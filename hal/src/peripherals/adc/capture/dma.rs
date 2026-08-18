use voltserver_hal::{
    adc::{Capture, ReadyCapture, InProgressCapture, CompleteCapture, RawSample,},
    dma::{ReadableDstBuffer},
};
use crate::adc::{Adc, AdcInstance, AdcStartMux, Flags, SampleMode, GND, PosChannel, NegChannel, PosAdcPin, NegAdcPin, sample::{Resolution, SignedSample, UnsignedSample}};
use core::marker::PhantomData;
use crate::typelevel::{Sealed, NoneT};

use crate::{dmac, evsys};
use dmac::transfer::State as TransferState;
use dmac::transfer::TransferChannelId;
use dmac::BufferPairBeat;

pub struct Freerun {}
impl evsys::OptionEvent for Freerun {}

impl Default for Freerun {
    fn default() -> Self {
        Freerun {}
    }
}

pub struct SingleEndedCapture<const N: usize, I, P, R, B, T, E = NoneT>
where
    I: AdcInstance,
    E: evsys::OptionEvent,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    adc: Adc<I>,
    dma_transfer: T,
    event: E,
    _pos: PhantomData<P>,
    _res: PhantomData<R>,
}


impl<const N: usize, I, P, R, B, T, E> Capture for SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance,
    E: evsys::OptionEvent,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    type Error = crate::adc::Error;
    type Sample = UnsignedSample<R>;
    type Output = [Self::Sample; N];
}

//==============================================================================
// Software-triggered SingleEndedCapture (E = NoneT)
//==============================================================================
impl<const N: usize, I, P, R, B, T> SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    /// Create a single-ended software-only triggered capture for an ADC channel
    fn from_channel(adc: Adc<I>, _pos: P, dma_transfer: T) -> Self {
        Self {
            adc,
            dma_transfer,
            event: NoneT::default(),
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    /// Create a single-ended software-only triggered capture for an ADC pin
    fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I>, _pin: Pin, dma_transfer: T) -> Self {
        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer)
    }

    /// Check for DMA or ADC peripheral error
    fn check_for_errors(&mut self) -> Result<(), <Self as Capture>::Error> {
        let adc_flags = self.adc.read_flags();
        self.adc.check_overrun(&adc_flags)?;
        self.dma_transfer.channel_error()?;

        Ok(())
    }

}

impl<const N: usize, I, P, R, B, T, BusyXfer> ReadyCapture for SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::ReadyTransfer<Buf = B, Busy = BusyXfer>,
    BusyXfer: dmac::BusyTransfer<Buf = B>,
{
    type InProgress = SingleEndedCapture<N, I, P, R, B, BusyXfer>;

    fn start(mut self) -> Result<Self::InProgress, Self::Error> {
        // Flush and configure the ADC, clearing any stale flags
        self.adc.disable_start_events();
        self.adc.flush();
        self.adc.disable_interrupts(Flags::all());
        self.adc.clear_all_flags();
        self.adc.disable_freerunning();
        self.adc.set_sample_mode(SampleMode::SingleEnded);
        self.adc.mux(P::MUXVAL, GND::<I>::MUXVAL);

        // Start DMA channel (& check for errors)
        let mut started_transfer = self.dma_transfer.begin();
        started_transfer.channel_error()?;

        // Start ADC (and check for errors)
        self.adc.enable_start_events();
        let adc_flags = self.adc.read_flags();
        self.adc.check_overrun(&adc_flags)?;

        Ok(Self::InProgress {
            adc: self.adc,
            dma_transfer: started_transfer,
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, P, R, B, T, CompleteXfer> InProgressCapture for SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::BusyTransfer<Buf = B, Complete = CompleteXfer>,
    CompleteXfer: dmac::CompleteTransfer<Buf = B>
{
    type Complete = SingleEndedCapture<N, I, P, R, B, CompleteXfer>;

    fn trigger(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        // Trigger the ADC directly, as there is no event (E = NoneT)
        self.adc.start_conversion();
        Ok(())
    }

    fn is_complete(&mut self) -> Result<bool, Self::Error> {
        self.check_for_errors()?;

        Ok(self.dma_transfer.is_complete())
    }

    fn wait(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        while !self.dma_transfer.is_complete() {}

        self.dma_transfer.channel_error()?;

        Ok(())
    }

    fn stop(mut self) -> Result<Self::Complete, Self::Error> {
        self.check_for_errors()?;

        Ok(Self::Complete {
            adc: self.adc,
            dma_transfer: self.dma_transfer.stop(),
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, P, R, B, T, ReadyXfer> CompleteCapture for SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::CompleteTransfer<Buf = B, Ready = ReadyXfer>,
    ReadyXfer: dmac::ReadyTransfer<Buf = B>
{
    type Ready = SingleEndedCapture<N, I, P, R, B, ReadyXfer>;

    fn reset(self) -> Self::Ready {
        Self::Ready {
            adc: self.adc,
            dma_transfer: self.dma_transfer.reset(),
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn convert(mut self) -> Result<(Self::Ready, Self::Output), Self::Error> {
        //TODO: can this use the unsafe from_array_unchecked? Do we need to verify samples coming
        // from DMA?
        //TODO: add logic to handle left-adjusted samples
        let samples = Self::Sample::from_array(self.dma_transfer.borrow_destination().read()).ok_or(Self::Error::SampleOverflow)?;

        Ok((self.reset(), samples))
    }
}

//==============================================================================
// Freerunning SingleEndedCapture (E = Freerun)
//==============================================================================
impl<const N: usize, I, P, R, B, T> SingleEndedCapture<N, I, P, R, B, T, Freerun>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    fn from_channel(adc: Adc<I>, _pos: P, dma_transfer: T) -> Self {
        Self {
            adc,
            dma_transfer,
            event: Freerun::default(),
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I>, _pin: Pin, dma_transfer: T) -> Self {
        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer)
    }

    fn check_for_errors(&mut self) -> Result<(), <Self as Capture>::Error> {
        let adc_flags = self.adc.read_flags();
        self.adc.check_overrun(&adc_flags)?;
        self.dma_transfer.channel_error()?;

        Ok(())
    }
}

impl<const N: usize, I, P, R, B, T, BusyXfer> ReadyCapture for SingleEndedCapture<N, I, P, R, B, T, Freerun>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::ReadyTransfer<Buf = B, Busy = BusyXfer>,
    BusyXfer: dmac::BusyTransfer<Buf = B>,
{
    type InProgress = SingleEndedCapture<N, I, P, R, B, BusyXfer, Freerun>;

    fn start(mut self) -> Result<Self::InProgress, Self::Error> {
        // Flush and configure the ADC, clearing any stale flags
        self.adc.disable_start_events();
        self.adc.flush();
        self.adc.disable_interrupts(Flags::all());
        self.adc.clear_all_flags();
        self.adc.disable_freerunning();
        self.adc.set_sample_mode(SampleMode::SingleEnded);
        self.adc.mux(P::MUXVAL, GND::<I>::MUXVAL);

        // Start DMA channel (& check for errors)
        let mut started_transfer = self.dma_transfer.begin();
        started_transfer.channel_error()?;

        // Configure the ADC for freerunning mode
        self.adc.enable_freerunning();

        Ok(Self::InProgress {
            adc: self.adc,
            dma_transfer: started_transfer,
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, P, R, B, T, CompleteXfer> InProgressCapture for SingleEndedCapture<N, I, P, R, B, T, Freerun>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::BusyTransfer<Buf = B, Complete = CompleteXfer>,
    CompleteXfer: dmac::CompleteTransfer<Buf = B>,
{
    type Complete = SingleEndedCapture<N, I, P, R, B, CompleteXfer, Freerun>;

    fn trigger(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        // Trigger the ADC directly (no event)
        // this should initiate the entire capture
        self.adc.start_conversion();
        Ok(())
    }

    fn is_complete(&mut self) -> Result<bool, Self::Error> {
        self.check_for_errors()?;

        Ok(self.dma_transfer.is_complete())
    }

    fn wait(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        while !self.dma_transfer.is_complete() {}

        self.check_for_errors()?;

        Ok(())
    }

    fn stop(mut self) -> Result<Self::Complete, Self::Error> {

        // Stop the ADC from starting any more conversions
        self.adc.disable_freerunning();

        let dma_complete = self.dma_transfer.is_complete();

        // Stop DMA transfer
        let mut transfer = self.dma_transfer.stop();

        // Check for DMA errors
        transfer.channel_error()?;

        // Check for ADC errors (if DMA hadn't already completed)
        if !dma_complete {
            let adc_flags = self.adc.read_flags();
            self.adc.check_overrun(&adc_flags)?;
        }

        // Clear all peripheral flags
        self.adc.clear_all_flags();
        //TODO: clear DMA channel flags?

        Ok(Self::Complete {
            adc: self.adc,
            dma_transfer: transfer,
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, P, R, B, T, ReadyXfer> CompleteCapture for SingleEndedCapture<N, I, P, R, B, T, Freerun>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::CompleteTransfer<Buf = B, Ready = ReadyXfer>,
    ReadyXfer: dmac::ReadyTransfer<Buf = B>
{
    type Ready = SingleEndedCapture<N, I, P, R, B, ReadyXfer, Freerun>;

    fn reset(self) -> Self::Ready {
        Self::Ready {
            adc: self.adc,
            dma_transfer: self.dma_transfer.reset(),
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn convert(mut self) -> Result<(Self::Ready, Self::Output), Self::Error> {
        //TODO: can this use the unsafe from_array_unchecked? Do we need to verify samples coming
        // from DMA?
        //TODO: add logic to handle left-adjusted samples
        let samples = Self::Sample::from_array(self.dma_transfer.borrow_destination().read()).ok_or(Self::Error::SampleOverflow)?;

        Ok((self.reset(), samples))
    }
}


//==============================================================================
// Event-triggered SingleEndedCapture
//==============================================================================
impl<const N: usize, I, M, E, P, R, B, T> SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance + evsys::User<M>,
    M: AdcStartMux<Instance = I>,
    E: evsys::AnyEvent<UserMux = M>,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count> + ReadableDstBuffer<BufferPairBeat<B>>>,
    T: dmac::ReadyTransfer<Buf = B>,
{
    fn from_channel(adc: Adc<I>, _pos: P, dma_transfer: T, event: E) -> Self {
        Self {
            adc,
            dma_transfer,
            event,
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I>, _pin: Pin, dma_transfer: T, event: E) -> Self {
        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer, event)
    }
}

impl<const N: usize, I, M, E, P, R, B, T, BusyXfer> ReadyCapture for SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance + evsys::User<M>,
    M: AdcStartMux<Instance = I>,
    E: evsys::AnyEvent<UserMux = M>,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::ReadyTransfer<Buf = B, Busy = BusyXfer>,
    BusyXfer: dmac::BusyTransfer<Buf = B>,
{
    type InProgress = SingleEndedCapture<N, I, P, R, B, BusyXfer, E>;

    fn start(mut self) -> Result<Self::InProgress, Self::Error> {
        // Flush and configure the ADC, clearng any stale flags
        self.adc.disable_start_events();
        self.adc.flush();
        self.adc.disable_interrupts(Flags::all());
        self.adc.clear_all_flags();
        self.adc.disable_freerunning();
        self.adc.set_sample_mode(SampleMode::SingleEnded);
        self.adc.mux(P::MUXVAL, GND::<I>::MUXVAL);

        // Start DMA first as to not miss any conversions
        let mut started_transfer = self.dma_transfer.begin();

        // Enable ADC START event input
        self.adc.enable_start_events();
        started_transfer.channel_error()?;

        let adc_flags = self.adc.read_flags();
        self.adc.check_overrun(&adc_flags)?;

        // Clear any pending errors on the event channel (overrun may have occured
        // since event source may already be generating events)
        self.event.clear_channel_errors();

        Ok(Self::InProgress {
            adc: self.adc,
            dma_transfer: started_transfer,
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, M, E, P, R, B, T> SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance + evsys::User<M>,
    M: AdcStartMux<Instance = I>,
    E: evsys::AnyEvent<UserMux = M>,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::BusyTransfer<Buf = B>,
{
    /// Check for DMA, ADC, or EVSYS peripheral errors
    fn check_for_errors(&mut self) -> Result<(), <Self as Capture>::Error> {
        // only check ADC overrun errors if the DMA transfer is still ongoing
        if !self.dma_transfer.is_complete() {
            let adc_flags = self.adc.read_flags();
            self.adc.check_overrun(&adc_flags)?;
        }
        self.dma_transfer.channel_error()?;
        self.event.channel_error()?;

        Ok(())
    }
}

impl<const N: usize, I, M, E, P, R, B, T, CompleteXfer> InProgressCapture for SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance + evsys::User<M>,
    M: AdcStartMux<Instance = I>,
    E: evsys::AnyEvent<UserMux = M>,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::BusyTransfer<Buf = B, Complete = CompleteXfer>,
    CompleteXfer: dmac::CompleteTransfer<Buf = B>
{
    type Complete = SingleEndedCapture<N, I, P, R, B, CompleteXfer, E>;

    fn trigger(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        // Trigger the ADC directly since its faster than triggering
        // through the event system
        self.adc.start_conversion();
        Ok(())
    }

    fn is_complete(&mut self) -> Result<bool, Self::Error> {
        self.check_for_errors()?;

        Ok(self.dma_transfer.is_complete())
    }

    fn wait(&mut self) -> Result<(), Self::Error> {
        self.check_for_errors()?;

        while !self.dma_transfer.is_complete() {}

        self.check_for_errors()?;

        Ok(())
    }

    fn stop(mut self) -> Result<Self::Complete, Self::Error> {
        let event_err = self.event.channel_error();

        // Stop the ADC from starting any more conversions
        self.adc.disable_start_events();

        // Stop DMA transfer
        let mut transfer = self.dma_transfer.stop();

        // check for event errors (prior to disabling ADC event input)
        event_err?;

        // Check for ADC errors
        let adc_flags = self.adc.read_flags();
        self.adc.check_overrun(&adc_flags)?;

        // Check for DMA errors
        transfer.channel_error()?;

        // Clear all peripheral flags
        self.adc.clear_all_flags();
        self.event.clear_channel_errors();
        //TODO: clear DMA channel flags?

        Ok(Self::Complete {
            adc: self.adc,
            dma_transfer: transfer,
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        })
    }
}

impl<const N: usize, I, M, E, P, R, B, T, ReadyXfer> CompleteCapture for SingleEndedCapture<N, I, P, R, B, T, E>
where
    I: AdcInstance + evsys::User<M>,
    M: AdcStartMux<Instance = I>,
    E: evsys::AnyEvent<UserMux = M>,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<
        Src = Adc<I>,
        Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>
            + ReadableDstBuffer<BufferPairBeat<B>,
                Contents = [<UnsignedSample<R> as RawSample>::Count; N]>
        >,
    T: dmac::CompleteTransfer<Buf = B, Ready = ReadyXfer>,
    ReadyXfer: dmac::ReadyTransfer<Buf = B>
{
    type Ready = SingleEndedCapture<N, I, P, R, B, ReadyXfer, E>;

    fn reset(self) -> Self::Ready {
        Self::Ready {
            adc: self.adc,
            dma_transfer: self.dma_transfer.reset(),
            event: self.event,
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn convert(mut self) -> Result<(Self::Ready, Self::Output), Self::Error> {
        //TODO: can this use the unsafe from_array_unchecked? Do we need to verify samples coming
        // from DMA?
        //TODO: add logic to handle left-adjusted samples
        let samples = Self::Sample::from_array(self.dma_transfer.borrow_destination().read()).ok_or(Self::Error::SampleOverflow)?;

        Ok((self.reset(), samples))
    }
}

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
