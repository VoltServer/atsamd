use voltserver_hal::{
    adc::{Capture, Sample, UnsignedSample, Resolution, ReadyCapture, ArmedCapture, BusyCapture, CompleteCapture},
    dma::{ReadableDstBuffer},
};
use crate::adc::{
    Adc, AdcResultBuffer, AdcInstance, Flags, SampleMode, GND, PosChannel, NegChannel,
    PosAdcPin, NegAdcPin, Accumulation, AccumulationResolution, Error,
};
use core::marker::PhantomData;
use core::cell::{RefCell, RefMut};
use crate::typelevel::{Sealed, NoneT};

use crate::{dmac, evsys};
use dmac::transfer::State as TransferState;
use dmac::{BufferPairBeat, AnyTransfer};
use evsys::AnyEvent;

//==============================================================================
// SingleEndedCaptureCfg
//==============================================================================
pub trait SingleEndedCaptureCfg {
    type Sample: Sample<CfgResolution<Self>> + 'static;
    type AdcInst: AdcInstance;
    type Accum: Accumulation;
    type PosCh: PosChannel<Self::AdcInst>;
    type DmaChannelId: dmac::ChId;
    type Event: evsys::AnyEvent<User: evsys::AnyUser<UserId = CfgAdcStartEventId<Self>>>;
}

pub type CfgResolution<T> = AccumulationResolution<<T as SingleEndedCaptureCfg>::Accum>;
type CfgAdcChannel<T> = <T as SingleEndedCaptureCfg>::PosCh;
type CfgAdcStartEventId<T> = <<T as SingleEndedCaptureCfg>::AdcInst as AdcInstance>::StartEventId;
type CfgDmaSourceBuffer<T> = AdcResultBuffer<<T as SingleEndedCaptureCfg>::AdcInst>;
type CfgDmaBufferPair<T, D> = dmac::BufferPair<CfgDmaSourceBuffer<T>, D>;
type CfgAdcInstance<T> = <T as SingleEndedCaptureCfg>::AdcInst;

//==============================================================================
// CaptureState
//==============================================================================
pub trait CaptureState {
    fn check_for_error(&mut self) -> Result<(), Error>;
}

pub struct Ready<const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> {
    dma_channel: dmac::channel::Channel<C::DmaChannelId, dmac::channel::Ready>,
    dma_buffer: &'static mut [C::Sample; LENGTH],
}

impl<const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> CaptureState for Ready<LENGTH, C> {
    fn check_for_error(&mut self) -> Result<(), Error> {
        // No errors to check for in this state
        Ok(())
    }
}

pub struct Armed<'a, const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> {
    pub adc: RefMut<'a, Adc<C::AdcInst, C::Accum>>,
    dma_transfer: dmac::transfer::Transfer<CfgDmaBufferPair<C, &'static mut [u16; LENGTH]>, dmac::transfer::Ready<C::DmaChannelId>>,
}

impl<const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> CaptureState for Armed<'_, LENGTH, C> {
    /// Checks for DMA errors
    fn check_for_error(&mut self) -> Result<(), Error> {
        self.dma_transfer.channel_error()?;
        Ok(())
    }
}

pub struct Busy<'a, const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> {
    pub adc: RefMut<'a, Adc<C::AdcInst, C::Accum>>,
    dma_transfer: dmac::transfer::Transfer<CfgDmaBufferPair<C, &'static mut [u16; LENGTH]>, dmac::transfer::Busy<C::DmaChannelId>>,
}

impl<const LENGTH: usize, C: SingleEndedCaptureCfg<Sample = u16>> CaptureState for Busy<'_, LENGTH, C> {
    /// Checks for DMA & ADC overrun errors
    fn check_for_error(&mut self) -> Result<(), Error> {
        self.dma_transfer.channel_error()?;

        if self.adc.read_flags().contains(Flags::OVERRUN) {
            Err(Error::BufferOverrun)
        } else {
            Ok(())
        }
    }
}

pub struct Complete<const LENGTH: usize, C: SingleEndedCaptureCfg> {
    dma_buffer: &'static mut [u16; LENGTH],
    dma_channel: dmac::channel::Channel<C::DmaChannelId, dmac::channel::Ready>,
    shift_amt: u32,
}

impl<const LENGTH: usize, C: SingleEndedCaptureCfg> CaptureState for Complete<LENGTH, C> {
    fn check_for_error(&mut self) -> Result<(), Error> {
        // No errors to check for in this state
        Ok(())
    }
}

//==============================================================================
// SingledEndedCapture
//==============================================================================
pub struct SingleEndedCapture<'a, const LENGTH: usize, C, S>
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    S: CaptureState,
{
    //TODO: should probably be a mutex
    adc: &'a RefCell<Adc<C::AdcInst, C::Accum>>,
    event: C::Event,
    oneshot: bool,
    state: S,
}

impl<'a, const LENGTH: usize, C, S> SingleEndedCapture<'a, LENGTH, C, S>
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    S: CaptureState,
{
    fn check_for_errors(&mut self) -> Result<(), Error> {
        self.state.check_for_error()?;
        self.event.channel_error()?;
        Ok(())
    }
}

impl<'a, const LENGTH: usize, C> SingleEndedCapture<'a, LENGTH, C, Ready<LENGTH, C>> 
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    u16: Sample<CfgResolution<C>>,
{
    pub fn arm<'adc>(self) -> Result<SingleEndedCapture<'a, LENGTH, C, Armed<'adc, LENGTH, C>>, Error>
    where
        'a: 'adc,
    {
        // Get mutable reference to ADC peripheral
        let mut adc = self.adc.try_borrow_mut().unwrap(); //TODO: handle error

        // Flush and configure the ADC, clearing any stale flags
        adc.disable_start_events();
        adc.disable_freerunning();
        adc.flush();
        adc.disable_interrupts(Flags::all());
        adc.clear_all_flags();
        adc.set_sample_mode(SampleMode::SingleEnded);
        adc.mux(
            CfgAdcChannel::<C>::MUXVAL,
            GND::<C::AdcInst>::MUXVAL
        );

        // Arm the DMA transfer
        let dma_transfer = dmac::transfer::Transfer::new(
            self.state.dma_channel,
            adc.dma_buffer_take().unwrap(), //TODO: handle error
            self.state.dma_buffer,
            false,
            CfgAdcInstance::<C>::DMA_RESRDY_TRIGGER,
            dmac::TriggerAction::Burst,
        ).unwrap(); //TODO: handle error

        let state = Armed {
            adc,
            dma_transfer, 
        };

        Ok(SingleEndedCapture {
            adc: self.adc,
            event: self.event,
            oneshot: self.oneshot,
            state,
        })
    }
}

impl<'a, 'adc, const LENGTH: usize, C> SingleEndedCapture<'a, LENGTH, C, Armed<'adc, LENGTH, C>>
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    u16: Sample<CfgResolution<C>>,
{
    pub fn start(mut self) -> Result<SingleEndedCapture<'a, LENGTH, C, Busy<'adc, LENGTH, C>>, Error> {
        self.check_for_errors()?;

        if self.oneshot {
            self.state.adc.enable_freerunning();
        }

        self.state.adc.clear_all_flags();
        self.state.dma_transfer.clear_all_flags();

        let dma_transfer = self.state.dma_transfer.begin();

        self.state.adc.enable_start_events();

        let state = Busy {
            adc: self.state.adc,
            dma_transfer,
        };

        let mut capture = SingleEndedCapture {
            adc: self.adc,
            event: self.event,
            oneshot: self.oneshot,
            state,
        };

        capture.check_for_errors()?;

        Ok(capture)
    }
}

impl<'a, const LENGTH: usize, C> SingleEndedCapture<'a, LENGTH, C, Busy<'_, LENGTH, C>>
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    u16: Sample<CfgResolution<C>>,
{
    pub fn is_complete(&mut self) -> Result<bool, Error> {
        self.check_for_errors()?;

        Ok(self.state.dma_transfer.is_complete())
    }

    pub fn wait(&mut self) -> Result<(), Error> {
        while !self.state.dma_transfer.is_complete() {
            self.check_for_errors()?;
        }

        self.check_for_errors()?;
        Ok(())
    }

    pub fn stop(mut self) -> Result<SingleEndedCapture<'a, LENGTH, C, Complete<LENGTH, C>>, Error> {
        self.state.adc.disable_start_events();
        self.state.adc.disable_freerunning();

        self.check_for_errors()?;

        let (dma_channel, src, dma_buffer) = self.state.dma_transfer.stop().free();

        self.state.adc.dma_buffer_return(src);

        let state = Complete {
            dma_channel,
            dma_buffer,
            shift_amt: self.state.adc.result_shift_amt(),
        };

        Ok(SingleEndedCapture {
            adc: self.adc,
            event: self.event,
            oneshot: self.oneshot,
            state,
        })
    }
}

impl<'a, const LENGTH: usize, C> SingleEndedCapture<'a, LENGTH, C, Complete<LENGTH, C>>
where
    C: SingleEndedCaptureCfg<Sample = u16>,
    u16: Sample<CfgResolution<C>>,
{
    pub fn convert(&self) -> Result<[C::Sample; LENGTH], Error> {
        let mut adjusted_samples = [0; LENGTH];

        for (i, sample) in self.state.dma_buffer.iter().enumerate() {
            adjusted_samples[i] = sample >> self.state.shift_amt;
        }

        Ok(adjusted_samples)
    }

    pub fn reset(self) -> SingleEndedCapture<'a, LENGTH, C, Ready<LENGTH, C>> {
        // Reset dma_buffer
        self.state.dma_buffer.as_mut_slice().fill(0);

        let state = Ready {
            dma_channel: self.state.dma_channel,
            dma_buffer: self.state.dma_buffer,
        };

        SingleEndedCapture {
            adc: self.adc,
            event: self.event,
            oneshot: self.oneshot,
            state,
        }
    }
}


//impl<'a, const LENGTH: usize, C, S> Capture<CfgResolution<C>> for SingleEndedCapture<'a, LENGTH, C, S>
//where
//    C: SingleEndedCaptureCfg<Sample = u16>,
//    S: CaptureState,
//    u16: Sample<CfgResolution<C>>,
//{
//    type Error = Error;
//    type Sample = u16;
//    type Output = [u16; LENGTH];
//}
//
//impl<'a, const LENGTH: usize, C> ReadyCapture<CfgResolution<C>> for SingleEndedCapture<'a, LENGTH, C, Ready<LENGTH, C>>
//where
//    C: SingleEndedCaptureCfg<Sample = u16>,
//    u16: Sample<CfgResolution<C>>,
//{
//    type Armed<'adc> = SingleEndedCapture<'a, LENGTH, C, Armed<'adc, LENGTH, C>> where Self: 'adc;
//
//    fn arm<'adc>(self) -> Result<SingleEndedCapture<'a, LENGTH, C, Armed<'adc, LENGTH, C>>, Error>
//    where
//        'a: 'adc
//    {
//        // Get mutable reference to ADC peripheral
//        let mut adc = self.adc.try_borrow_mut().unwrap(); //TODO: handle error
//
//        // Flush and configure the ADC, clearing any stale flags
//        adc.disable_start_events();
//        adc.disable_freerunning();
//        adc.flush();
//        adc.disable_interrupts(Flags::all());
//        adc.clear_all_flags();
//        adc.set_sample_mode(SampleMode::SingleEnded);
//        adc.mux(
//            CfgAdcChannel::<C>::MUXVAL,
//            GND::<C::AdcInst>::MUXVAL
//        );
//
//        // Arm the DMA transfer
//        let dma_transfer = dmac::transfer::Transfer::new(
//            self.state.dma_channel,
//            adc.dma_buffer_take().unwrap(), //TODO: handle error
//            self.state.dma_buffer,
//            false,
//            CfgAdcInstance::<C>::DMA_RESRDY_TRIGGER,
//            dmac::TriggerAction::Burst,
//        ).unwrap(); //TODO: handle error
//
//        let state = Armed {
//            adc,
//            dma_transfer, 
//        };
//
//        Ok(SingleEndedCapture {
//            adc: self.adc,
//            event: self.event,
//            oneshot: self.oneshot,
//            state,
//        })
//    }
//}
//
//impl<'a, 'adc, const LENGTH: usize, C> ArmedCapture<CfgResolution<C>> for SingleEndedCapture<'a, LENGTH, C, Armed<'adc, LENGTH, C>>
//where
//    C: SingleEndedCaptureCfg<Sample = u16>,
//    u16: Sample<CfgResolution<C>>,
//{
//    type Busy = SingleEndedCapture<'a, LENGTH, C, Busy<'adc, LENGTH, C>>;
//
//    fn start(mut self) -> Result<SingleEndedCapture<'a, LENGTH, C, Busy<'adc, LENGTH, C>>, Error> {
//        self.check_for_errors()?;
//
//        if self.oneshot {
//            self.state.adc.enable_freerunning();
//        }
//
//        self.state.adc.clear_all_flags();
//        self.state.dma_transfer.clear_all_flags();
//
//        let dma_transfer = self.state.dma_transfer.begin();
//
//        self.state.adc.enable_start_events();
//
//        let state = Busy {
//            adc: self.state.adc,
//            dma_transfer,
//        };
//
//        let mut capture = SingleEndedCapture {
//            adc: self.adc,
//            event: self.event,
//            oneshot: self.oneshot,
//            state,
//        };
//
//        capture.check_for_errors()?;
//
//        Ok(capture)
//    }
//}
//
//impl <'a, const LENGTH: usize, C> BusyCapture<CfgResolution<C>> for SingleEndedCapture<'a, LENGTH, C, Busy<'_, LENGTH, C>>
//where
//    C: SingleEndedCaptureCfg<Sample = u16>,
//    u16: Sample<CfgResolution<C>>,
//{
//    type Complete = SingleEndedCapture<'a, LENGTH, C, Complete<LENGTH, C>>;
//
//    fn is_complete(&mut self) -> Result<bool, Error> {
//        self.check_for_errors()?;
//
//        Ok(self.state.dma_transfer.is_complete())
//    }
//
//    fn wait(&mut self) -> Result<(), Error> {
//        while !self.state.dma_transfer.is_complete() {
//            self.check_for_errors()?;
//        }
//
//        self.check_for_errors()?;
//        Ok(())
//    }
//
//    fn stop(mut self) -> Result<SingleEndedCapture<'a, LENGTH, C, Complete<LENGTH, C>>, Error> {
//        self.state.adc.disable_start_events();
//        self.state.adc.disable_freerunning();
//
//        self.check_for_errors()?;
//
//        let (dma_channel, src, dma_buffer) = self.state.dma_transfer.stop().free();
//
//        self.state.adc.dma_buffer_return(src);
//
//        let state = Complete {
//            dma_channel,
//            dma_buffer,
//            shift_amt: self.state.adc.result_shift_amt(),
//        };
//
//        Ok(SingleEndedCapture {
//            adc: self.adc,
//            event: self.event,
//            oneshot: self.oneshot,
//            state,
//        })
//    }
//}
//
//impl<'a, const LENGTH: usize, C> CompleteCapture<CfgResolution<C>> for SingleEndedCapture<'a, LENGTH, C, Complete<LENGTH, C>>
//where
//    C: SingleEndedCaptureCfg<Sample = u16>,
//    u16: Sample<CfgResolution<C>>,
//{
//    type Ready = SingleEndedCapture<'a, LENGTH, C, Ready<LENGTH, C>>;
//
//    fn convert(&self) -> Result<[C::Sample; LENGTH], Error> {
//        let mut adjusted_samples = [0; LENGTH];
//
//        for (i, sample) in self.state.dma_buffer.iter().enumerate() {
//            adjusted_samples[i] = sample >> self.state.shift_amt;
//        }
//
//        Ok(adjusted_samples)
//    }
//
//    fn reset(self) -> SingleEndedCapture<'a, LENGTH, C, Ready<LENGTH, C>> {
//        // Reset dma_buffer
//        self.state.dma_buffer.as_mut_slice().fill(0);
//
//        let state = Ready {
//            dma_channel: self.state.dma_channel,
//            dma_buffer: self.state.dma_buffer,
//        };
//
//        SingleEndedCapture {
//            adc: self.adc,
//            event: self.event,
//            oneshot: self.oneshot,
//            state,
//        }
//    }
//}












//impl<const LENGTH: usize, C> SingleEndedCapture<LENGTH, C>
//where
//    C: SingleEndedCaptureCfg<LENGTH, Event = NoneT>,
//{
//    fn check_for_errors(&mut self) -> Result<(), Error> {
//        todo!()
//    }
//}
//
//impl<const LENGHT: usize, C> SingleEndedCapture<LENGTH, C>
//
//
//
//// Capture Driver for software-triggered captures
//impl<const LENGTH: usize, C> CaptureDriver<CfgResolution<LENGTH, C>> for SingleEndedCapture<LENGTH, C>
//where
//    C: SingleEndedCaptureCfg<LENGTH, Event = NoneT>,
//    u16: Sample<AccumulationResolution<C::Accum>>,
//{
//    type Error = Error;
//    type Sample = u16;
//    type Output = [u16; LENGTH];
//
//    unsafe fn init(&mut self) -> Result<(), Error> {
//        todo!()
//    }
//
//    unsafe fn start(&mut self) -> Result<(), Error> {
//        // Flush and configure the ADC, clearing any stale flags
//        self.adc.disable_start_events();
//        self.adc.disable_freerunning();
//        self.adc.flush();
//        self.adc.disable_interrupts(Flags::all());
//        self.adc.clear_all_flags();
//        self.adc.set_sample_mode(SampleMode::SingleEnded);
//        self.adc.mux(
//            <<C as SingleEndedCaptureCfg<LENGTH>>::PosCh as PosChannel<C::AdcInst>>::MUXVAL,
//            GND::<C::AdcInst>::MUXVAL
//        );
//
//        // Start DMA channel (& check for errors)
//        self.dma_transfer.clear_channel_errors();
//        //TODO: start DMA
//        //let mut started_transfer = self.dma_transfer.begin();
//        //started_transfer.channel_error()?;
//
//        // Enable freerunning (if confgured for oneshot capture)
//        if self.oneshot {
//            self.adc.enable_freerunning()
//        }
//
//        Ok(())
//    }
//
//    unsafe fn is_complete(&mut self) -> Result<bool, Error> {
//        todo!()
//    }
//
//    unsafe fn wait(&mut self) -> Result<(), Error> {
//        todo!()
//    }
//
//    unsafe fn stop(&mut self) -> Result<(), Error> {
//        self.adc.disable_start_events();
//        self.adc.disable_freerunning();
//        self.adc.flush();
//        self.adc.disable_interrupts(Flags::all());
//
//        self.check_for_errors()
//    }
//
//    unsafe fn convert(&self) -> Result<[u16; LENGTH], Error> {
//        //let samples = Self::Sample::from_array(
//        //        self.dma_transfer.borrow_destination().read(),
//        //        self.adc.check_left_adjust(),
//        //    )
//        //    .ok_or(Self::Error::SampleOverflow)?;
//    }
//
//    unsafe fn reset(&mut self) {
//        todo!()
//    }
//}










//impl<const N: usize, S, I, A, P, B, T, E> Capture<AccumulationResolution<A>> for SingleEndedCapture<N, S, I, A, P, B, T, E>
//where
//    S: State,
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//    E: evsys::OptionEvent,
//{
//    type Error = crate::adc::Error;
//    type Sample = u16;
//    type Output = [Self::Sample; N];
//}
//
////==============================================================================
//// Software-triggered SingleEndedCapture (E = NoneT)
////==============================================================================
//impl<const N: usize, I, A, P, B, T> SingleEndedCapture<N, Ready, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    /// Create a single-ended software-only triggered capture for an ADC channel
//    pub(crate) fn from_channel(adc: Adc<I, A>, _pos: P, dma_transfer: T, oneshot: bool) -> Self {
//        Self {
//            adc,
//            dma_transfer,
//            event: NoneT::default(),
//            oneshot,
//            _pos: PhantomData,
//            _state: PhantomData,
//        }
//    }
//
//    /// Create a single-ended software-only triggered capture for an ADC pin
//    pub(crate) fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I, A>, _pin: Pin, dma_transfer: T, oneshot: bool) -> Self {
//        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer, oneshot)
//    }
//
//    /// Check for DMA or ADC peripheral error
//    fn check_for_errors(&mut self) -> Result<(), <Self as Capture<AccumulationResolution<A>>>::Error> {
//        let adc_flags = self.adc.read_flags();
//        self.adc.check_overrun(&adc_flags)?;
//        self.dma_transfer.channel_error()?;
//
//        Ok(())
//    }
//}
//
//impl<'a, const N: usize, I, A, P, B, T> ReadyCapture<'a, AccumulationResolution<A>> for SingleEndedCapture<N, Ready, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    type InProgress = SingleEndedCapture<N, InProgress<'a>, I, A, P, B, T>;
//
//    fn start(&'a mut self) -> Result<Self::InProgress, Self::Error> {
//        todo!()
//    }
//}
//
//impl<'a, const N: usize, I, A, P, B, T> InProgressCapture<'a, AccumulationResolution<A>> for SingleEndedCapture<N, InProgress<'a>, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    type Complete = SingleEndedCapture<N, Complete<'a>, I, A, P, B, T>;
//
//    fn is_complete(&self) -> Result<bool, Self::Error> {
//        todo!()
//    }
//
//    fn wait(&mut self) -> Result<(), Self::Error> {
//        todo!()
//    }
//
//    fn stop(&'a mut self) -> Result<Self::Complete, Self::Error> {
//        todo!()
//    }
//}
//
//impl<'a, const N: usize, I, A, P, B, T> CompleteCapture<'a, AccumulationResolution<A>> for SingleEndedCapture<N, Complete<'a>, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    type Ready = SingleEndedCapture<N, Ready, I, A, P, B, T>;
//
//    fn convert(&'a self) -> Result<&'a Self::Output, Self::Error> {
//        todo!()
//    }
//
//    fn reset(&'a mut self) -> Self::Ready {
//        todo!()
//    }
//
//    fn free(self) {
//        todo!()
//    }
//}





//pub struct SingleEndedCapture<const N: usize, I, A, P, B, T, E = NoneT>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    E: evsys::OptionEvent,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<AccumulationResolution<A>> + UnsignedSample<AccumulationResolution<A>>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    adc: Adc<I, A>,
//    dma_transfer: T,
//    event: E,
//    oneshot: bool,
//    _pos: PhantomData<P>,
//}


//impl<const N: usize, I, A, P, B, T, E> Capture for SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    E: evsys::OptionEvent,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    type Error = crate::adc::Error;
//    type Sample = UnsignedSample<AccumulationResolution<A>>;
//    type Output = [Self::Sample; N];
//}
//
////==============================================================================
//// Software-triggered SingleEndedCapture (E = NoneT)
////==============================================================================
//impl<const N: usize, I, A, P, B, T> SingleEndedCapture<N, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::AnyTransfer<Buf = B>,
//{
//    /// Create a single-ended software-only triggered capture for an ADC channel
//    pub(crate) fn from_channel(adc: Adc<I, A>, _pos: P, dma_transfer: T, oneshot: bool) -> Self {
//        Self {
//            adc,
//            dma_transfer,
//            event: NoneT::default(),
//            oneshot,
//            _pos: PhantomData,
//        }
//    }
//
//    /// Create a single-ended software-only triggered capture for an ADC pin
//    pub(crate) fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I, A>, _pin: Pin, dma_transfer: T, oneshot: bool) -> Self {
//        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer, oneshot)
//    }
//
//    /// Check for DMA or ADC peripheral error
//    fn check_for_errors(&mut self) -> Result<(), <Self as Capture>::Error> {
//        let adc_flags = self.adc.read_flags();
//        self.adc.check_overrun(&adc_flags)?;
//        self.dma_transfer.channel_error()?;
//
//        Ok(())
//    }
//
//}
//
//impl<const N: usize, I, A, P, B, T, BusyXfer> ReadyCapture for SingleEndedCapture<N, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::ReadyTransfer<Buf = B, Busy = BusyXfer>,
//    BusyXfer: dmac::BusyTransfer<Buf = B>,
//{
//    type InProgress = SingleEndedCapture<N, I, A, P, B, BusyXfer>;
//
//    fn start(mut self) -> Result<Self::InProgress, Self::Error> {
//        // Flush and configure the ADC, clearing any stale flags
//        self.adc.disable_start_events();
//        self.adc.disable_freerunning();
//        self.adc.flush();
//        self.adc.disable_interrupts(Flags::all());
//        self.adc.clear_all_flags();
//        self.adc.set_sample_mode(SampleMode::SingleEnded);
//        self.adc.mux(P::MUXVAL, GND::<I>::MUXVAL);
//
//        // Start DMA channel (& check for errors)
//        self.dma_transfer.clear_channel_errors();
//        let mut started_transfer = self.dma_transfer.begin();
//        started_transfer.channel_error()?;
//
//        // Enable freerunning (if confgured for oneshot capture)
//        if self.oneshot {
//            self.adc.enable_freerunning()
//        }
//
//        Ok(Self::InProgress {
//            adc: self.adc,
//            dma_transfer: started_transfer,
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        })
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, CompleteXfer> InProgressCapture for SingleEndedCapture<N, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::BusyTransfer<Buf = B, Complete = CompleteXfer>,
//    CompleteXfer: dmac::CompleteTransfer<Buf = B>
//{
//    type Complete = SingleEndedCapture<N, I, A, P, B, CompleteXfer>;
//
//    fn trigger(&mut self) -> Result<(), Self::Error> {
//        self.check_for_errors()?;
//
//        // Trigger the ADC directly, as there is no event (E = NoneT)
//        self.adc.start_conversion();
//        Ok(())
//    }
//
//    fn is_complete(&mut self) -> Result<bool, Self::Error> {
//        self.check_for_errors()?;
//
//        Ok(self.dma_transfer.is_complete())
//    }
//
//    fn wait(&mut self) -> Result<(), Self::Error> {
//        self.check_for_errors()?;
//
//        while !self.dma_transfer.is_complete() {}
//
//        self.dma_transfer.channel_error()?;
//
//        Ok(())
//    }
//
//    fn stop(mut self) -> Result<Self::Complete, Self::Error> {
//        self.adc.disable_start_events();
//        self.adc.disable_freerunning();
//        self.adc.flush();
//        self.adc.disable_interrupts(Flags::all());
//
//        self.check_for_errors()?;
//
//        Ok(Self::Complete {
//            adc: self.adc,
//            dma_transfer: self.dma_transfer.stop(),
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        })
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, ReadyXfer> CompleteCapture for SingleEndedCapture<N, I, A, P, B, T>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::CompleteTransfer<Buf = B, Ready = ReadyXfer>,
//    ReadyXfer: dmac::ReadyTransfer<Buf = B>
//{
//    type Ready = SingleEndedCapture<N, I, A, P, B, ReadyXfer>;
//
//    fn reset(self) -> Self::Ready {
//        Self::Ready {
//            adc: self.adc,
//            dma_transfer: self.dma_transfer.reset(),
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        }
//    }
//
//    fn convert(mut self) -> Result<(Self::Ready, Self::Output), Self::Error> {
//        //TODO: can this use the unsafe from_array_unchecked? Do we need to verify samples coming
//        // from DMA?
//        let samples = Self::Sample::from_array(
//                self.dma_transfer.borrow_destination().read(),
//                self.adc.check_left_adjust(),
//            )
//            .ok_or(Self::Error::SampleOverflow)?;
//
//        Ok((self.reset(), samples))
//    }
//}
//
////==============================================================================
//// Event-triggered SingleEndedCapture
////==============================================================================
//impl<const N: usize, I, A, P, B, T, E> SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::ReadyTransfer<Buf = B>,
//    E: evsys::AnyEvent<User: evsys::AnyUser<UserId = I::StartEventId>>,
//{
//    pub fn from_channel(adc: Adc<I, A>, _pos: P, dma_transfer: T, event: E, oneshot: bool) -> Self {
//        Self {
//            adc,
//            dma_transfer,
//            event,
//            oneshot,
//            _pos: PhantomData,
//        }
//    }
//
//    pub fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I, A>, _pin: Pin, dma_transfer: T, event: E, oneshot: bool) -> Self {
//        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer, event, oneshot)
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, E, BusyXfer> ReadyCapture for SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::ReadyTransfer<Buf = B, Busy = BusyXfer>,
//    E: evsys::AnyEvent<User: evsys::AnyUser<UserId = I::StartEventId>>,
//    BusyXfer: dmac::BusyTransfer<Buf = B>,
//{
//    type InProgress = SingleEndedCapture<N, I, A, P, B, BusyXfer, E>;
//
//    fn start(mut self) -> Result<Self::InProgress, Self::Error> {
//        // Flush and configure the ADC, clearng any stale flags
//        self.adc.disable_start_events();
//        self.adc.flush();
//        self.adc.disable_interrupts(Flags::all());
//        self.adc.clear_all_flags();
//        self.adc.disable_freerunning();
//        self.adc.set_sample_mode(SampleMode::SingleEnded);
//        self.adc.mux(P::MUXVAL, GND::<I>::MUXVAL);
//
//        // Start DMA first as to not miss any conversions
//        self.dma_transfer.clear_channel_errors();
//        let mut started_transfer = self.dma_transfer.begin();
//
//        // Enable freerunning if configured for oneshot capture
//        if self.oneshot {
//            self.adc.enable_freerunning()
//        }
//
//        // Enable ADC START event input
//        self.adc.enable_start_events();
//        started_transfer.channel_error()?;
//
//        // Check for any ADC errors
//        let adc_flags = self.adc.read_flags();
//        self.adc.check_overrun(&adc_flags)?;
//
//        // Clear any pending errors on the event channel (overrun may have occured
//        // since event source may already be generating events)
//        self.event.clear_channel_errors();
//
//        Ok(Self::InProgress {
//            adc: self.adc,
//            dma_transfer: started_transfer,
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        })
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, E> SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::BusyTransfer<Buf = B>,
//    E: evsys::AnyEvent<User: evsys::AnyUser<UserId = I::StartEventId>>,
//{
//    /// Check for DMA, ADC, or EVSYS peripheral errors
//    fn check_for_errors(&mut self) -> Result<(), <Self as Capture>::Error> {
//        // only check ADC overrun errors if the DMA transfer is still ongoing
//        if !self.dma_transfer.is_complete() {
//            let adc_flags = self.adc.read_flags();
//            self.adc.check_overrun(&adc_flags)?;
//        }
//        self.dma_transfer.channel_error()?;
//        self.event.channel_error()?;
//
//        Ok(())
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, E, CompleteXfer> InProgressCapture for SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::BusyTransfer<Buf = B, Complete = CompleteXfer>,
//    E: evsys::AnyEvent<User: evsys::AnyUser<UserId = I::StartEventId>>,
//    CompleteXfer: dmac::CompleteTransfer<Buf = B>
//{
//    type Complete = SingleEndedCapture<N, I, A, P, B, CompleteXfer, E>;
//
//    fn trigger(&mut self) -> Result<(), Self::Error> {
//        self.check_for_errors()?;
//
//        // Trigger the ADC directly since its faster than triggering
//        // through the event system
//        self.adc.start_conversion();
//        Ok(())
//    }
//
//    fn is_complete(&mut self) -> Result<bool, Self::Error> {
//        self.check_for_errors()?;
//
//        Ok(self.dma_transfer.is_complete())
//    }
//
//    fn wait(&mut self) -> Result<(), Self::Error> {
//        self.check_for_errors()?;
//
//        while !self.dma_transfer.is_complete() {}
//
//        self.check_for_errors()?;
//
//        Ok(())
//    }
//
//    fn stop(mut self) -> Result<Self::Complete, Self::Error> {
//        let event_err = self.event.channel_error();
//
//        // Stop the ADC from starting any more conversions
//        self.adc.disable_start_events();
//
//        // Stop DMA transfer
//        let mut transfer = self.dma_transfer.stop();
//
//        // check for event errors (prior to disabling ADC event input)
//        event_err?;
//
//        // Check for ADC errors
//        let adc_flags = self.adc.read_flags();
//        self.adc.check_overrun(&adc_flags)?;
//
//        // Check for DMA errors
//        transfer.channel_error()?;
//
//        // Clear all peripheral flags
//        self.adc.clear_all_flags();
//        self.event.clear_channel_errors();
//        transfer.clear_channel_errors();
//
//        Ok(Self::Complete {
//            adc: self.adc,
//            dma_transfer: transfer,
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        })
//    }
//}
//
//impl<const N: usize, I, A, P, B, T, E, ReadyXfer> CompleteCapture for SingleEndedCapture<N, I, A, P, B, T, E>
//where
//    I: AdcInstance,
//    A: Accumulation,
//    P: PosChannel<I>,
//    B: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst: dmac::Buffer<Beat: Sample<A::RES> + UnsignedSample<A::RES>>>,
//    T: dmac::CompleteTransfer<Buf = B, Ready = ReadyXfer>,
//    E: evsys::AnyEvent<User: evsys::AnyUser<UserId = I::StartEventId>>,
//    ReadyXfer: dmac::ReadyTransfer<Buf = B>
//{
//    type Ready = SingleEndedCapture<N, I, A, P, B, ReadyXfer, E>;
//
//    fn reset(self) -> Self::Ready {
//        Self::Ready {
//            adc: self.adc,
//            dma_transfer: self.dma_transfer.reset(),
//            event: self.event,
//            oneshot: self.oneshot,
//            _pos: PhantomData,
//        }
//    }
//
//    fn convert(mut self) -> Result<(Self::Ready, Self::Output), Self::Error> {
//        //TODO: can this use the unsafe from_array_unchecked? Do we need to verify samples coming
//        // from DMA?
//        let samples = Self::Sample::from_array(
//                self.dma_transfer.borrow_destination().read(),
//                self.adc.check_left_adjust(),
//            )
//            .ok_or(Self::Error::SampleOverflow)?;
//
//        Ok((self.reset(), samples))
//    }
//}

//pub struct DifferentialCapture<const N: usize, I, PC, NC, R, S>
//where
//    I: AdcInstance,
//    PC: PosChannel<I>,
//    NC: NegChannel<I>,
//    R: Resolution,
//    S: State,
//{
//    adc: Adc<I>,
//    buffer: [SignedSample<R>; N],
//    _pos: PhantomData<PC>,
//    _neg: PhantomData<NC>,
//    _state: PhantomData<S>,
//}
//
//impl<const N: usize, I, PC, NC, R, S> Capture for DifferentialCapture<N, I, PC, NC, R, S>
//where
//    I: AdcInstance,
//    PC: PosChannel<I>,
//    NC: NegChannel<I>,
//    R: Resolution,
//    S: State,
//{
//    type Error = super::Error;
//    type Sample = SignedSample<R>;
//    type Output = [Self::Sample; N];
//}
//
//impl<const N: usize, I, PC, NC, R> ReadyCapture for DifferentialCapture<N, I, PC, NC, R, Ready>
//where
//    I: AdcInstance,
//    PC: PosChannel<I>,
//    NC: NegChannel<I>,
//    R: Resolution,
//{
//    type InProgress = DifferentialCapture<N, I, PC, NC, R, InProgress>;
//
//    fn start(self) -> Result<Self::InProgress, Self::Error> {
//        todo!()
//    }
//}
//
//impl<const N: usize, I, PC, NC, R> InProgressCapture for DifferentialCapture<N, I, PC, NC, R, InProgress>
//where
//    I: AdcInstance,
//    PC: PosChannel<I>,
//    NC: NegChannel<I>,
//    R: Resolution,
//{
//    type Ready = DifferentialCapture<N, I, PC, NC, R, Ready>;
//    type Complete = DifferentialCapture<N, I, PC, NC, R, Complete>;
//
//    fn trigger(&mut self) -> Result<(), Self::Error> {
//        todo!()
//    }
//
//    fn is_complete(&self) -> Result<bool, Self::Error> {
//        todo!()
//    }
//
//    fn wait(&self) -> Result<(), Self::Error> {
//        todo!()
//    }
//
//    fn stop(self) -> Result<Self::Complete, Self::Error> {
//        todo!()
//    }
//}
//
//impl<const N: usize, I, PC, NC, R> CompleteCapture for DifferentialCapture<N, I, PC, NC, R, Complete>
//where
//    I: AdcInstance,
//    PC: PosChannel<I>,
//    NC: NegChannel<I>,
//    R: Resolution,
//{
//    type Ready = DifferentialCapture<N, I, PC, NC, R, Ready>;
//
//    fn reset(self) -> Self::Ready {
//        todo!()
//    }
//
//    fn convert(self) -> Result<(Self::Ready, Self::Output), Self::Error> {
//        todo!()
//    }
//}
