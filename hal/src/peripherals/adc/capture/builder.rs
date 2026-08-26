//! Builder to ease creation of [`Capture`]s
//!
//!

use crate::{dmac, evsys};
use voltserver_hal::adc::RawSample;
use crate::typelevel::{NoneT, Sealed};
use super::{
    dma::SingleEndedCapture as DmaSingleEndedCapture,
    super::{
        Resolution, _8Bit, _10Bit, _12Bit, _13Bit, _14Bit, _15Bit, _16Bit, PosChannel,
        AdcInstance, Adc, Accumulation, AccumulationResolution, AdcResultBuffer,
        sample::{UnsignedSample, SignedSample},
    },
};
use core::marker::PhantomData;

pub enum BuilderError {
    DmaBufferUnavailable,
    Dma(dmac::Error),
    Evsys(evsys::Error),
}

impl From<dmac::Error> for BuilderError {
    fn from(err: dmac::Error) -> Self {
        Self::Dma(err)
    }
}

impl From<evsys::Error> for BuilderError {
    fn from(err: evsys::Error) -> Self {
        Self::Evsys(err)
    }
}

pub trait OptionSample {}
impl<R: RawSample> OptionSample for R {}
impl OptionSample for NoneT {}

pub trait CaptureType: Sealed {
    type Sample: OptionSample;
}
pub struct SingleEnded<R: Resolution> {
    _res: PhantomData<R>,
}
pub struct Differential<R: Resolution> {
    _res: PhantomData<R>,
}

impl<R: Resolution> Sealed for SingleEnded<R> {}
impl<R: Resolution> Sealed for Differential<R> {}

impl<R: Resolution> CaptureType for SingleEnded<R> {
    type Sample = UnsignedSample<R>;
}
impl<R: Resolution> CaptureType for Differential<R> {
    type Sample = SignedSample<R>;
}
impl CaptureType for NoneT {
    type Sample = NoneT;
}

pub trait OptionBuffer {}
impl<B: dmac::Buffer> OptionBuffer for B {}
impl OptionBuffer for NoneT {}

pub trait TriggerMode: Sealed {}
#[derive(Default)]
pub struct Software {}
impl Sealed for Software {}
impl<EC: evsys::channel::AnyChannel> TriggerMode for EC {}
impl TriggerMode for Software {}
impl TriggerMode for NoneT {}

pub struct CaptureBuilder<R = _12Bit, T = NoneT, TM = NoneT, C = NoneT, B = NoneT, const N: usize = 0> 
where
    R: Resolution,
    T: CaptureType,
    TM: TriggerMode,
    C: dmac::channel::OptionChannel,
    B: OptionBuffer,
{
    oneshot: bool,
    trigger_mode: TM,
    dma_channel: C,
    dst_buffer: B,
    _type: PhantomData<T>,
    _resolution: PhantomData<R>,
}

impl CaptureBuilder {
    /// Create an 8-bit capture
    pub fn _8_bit() -> CaptureBuilder<_8Bit> {
        CaptureBuilder::default()
    }

    /// Create a 10-bit capture
    pub fn _10_bit() -> CaptureBuilder<_10Bit> {
        CaptureBuilder::default()
    }

    /// Create a 12-bit capture
    pub fn _12_bit() -> CaptureBuilder<_12Bit> {
        CaptureBuilder::default() 
    }

    /// Create a 13-bit capture
    ///
    /// # Note
    /// This requires configuring the ADC to use summation and/or
    /// oversampling to acheive, which effectivly combines multiple 12-bit
    /// samples together to acheive higher resolution
    pub fn _13_bit() -> CaptureBuilder<_13Bit> {
        CaptureBuilder::default()
    }

    /// Create a 14-bit capture
    ///
    /// # Note
    /// This requires configuring the ADC to use summation and/or
    /// oversampling to acheive, which effectivly combines multiple 12-bit
    /// samples together to acheive higher resolution
    pub fn _14_bit() -> CaptureBuilder<_14Bit> {
        CaptureBuilder::default()
    }

    /// Create a 15-bit capture
    ///
    /// # Note
    /// This requires configuring the ADC to use summation and/or
    /// oversampling to acheive, which effectivly combines multiple 12-bit
    /// samples together to acheive higher resolution
    pub fn _15_bit() -> CaptureBuilder<_15Bit> {
        CaptureBuilder::default()
    }

    /// Create a 16-bit capture
    ///
    /// # Note
    /// This requires configuring the ADC to use summation and/or
    /// oversampling to acheive, which effectivly combines multiple 12-bit
    /// samples together to acheive higher resolution
    pub fn _16_bit() -> CaptureBuilder<_16Bit> {
        CaptureBuilder::default()
    }
}

impl<R, T, TM, C, B, const N: usize> CaptureBuilder<R, T, TM, C, B, N>
where
    R: Resolution,
    T: CaptureType,
    TM: TriggerMode,
    C: dmac::channel::OptionChannel,
    B: OptionBuffer,
{
    /// Configure the trigger behavior of the capture.
    ///
    /// When oneshot triggering is enabled, a single trigger on the capture
    /// will cause all of the sample to be captured one after another.
    /// When disabled, each trigger captures exactly one sample.
    pub fn oneshot(mut self, yes: bool) -> Self {
        self.oneshot = yes;
        self
    }

    /// Set the capture type to be a single-ended capture
    /// Returns only positive values, referenced to GND
    pub fn single_ended(self) -> CaptureBuilder<R, SingleEnded<R>, TM, C, B> {
        CaptureBuilder {
            oneshot: self.oneshot,
            trigger_mode: self.trigger_mode,
            dma_channel: self.dma_channel,
            dst_buffer: self.dst_buffer,
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }

    /// Set the capture type to be a differential capture
    /// Returns positive or negative values, both positive and negative inputs
    /// can be specified.
    pub fn differential(self) -> CaptureBuilder<R, Differential<R>, TM, C, B> {
        CaptureBuilder {
            oneshot: self.oneshot,
            trigger_mode: self.trigger_mode,
            dma_channel: self.dma_channel,
            dst_buffer: self.dst_buffer,
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }

    /// Configure the capture to trigger from an event via [`evsys`]
    pub fn event_triggered<EC>(self, channel: EC) -> CaptureBuilder<R, T, EC, C, B>
    where
        EC: evsys::AnyChannel<Status = evsys::Asynchronous>,
    {
        CaptureBuilder {
            oneshot: self.oneshot,
            trigger_mode: channel,
            dma_channel: self.dma_channel,
            dst_buffer: self.dst_buffer,
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }

    /// Configure the capture to trigger via software request
    pub fn software_triggered(self) -> CaptureBuilder<R, T, Software, C, B> {
        CaptureBuilder {
            oneshot: self.oneshot,
            trigger_mode: Software::default(),
            dma_channel: self.dma_channel,
            dst_buffer: self.dst_buffer,
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }

    /// Configure the capture to use DMA to move samples from the ADC peripheral to 
    /// the destination buffer
    pub fn dma<DC, DB, S, const J: usize>(self, dma_channel: DC, dst_buffer: DB) -> CaptureBuilder<R, T, TM, DC, DB, J>
    where
        DC: dmac::channel::AnyChannel<Status = dmac::channel::Ready>,
        DB: dmac::Buffer<Beat = <T::Sample as RawSample>::Count>,
        T::Sample: RawSample,
    {
        CaptureBuilder {
            oneshot: self.oneshot,
            trigger_mode: self.trigger_mode,
            dma_channel,
            dst_buffer,
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }
}

impl<R, C, B, const N: usize> CaptureBuilder<R, SingleEnded<R>, Software, C, B, N>
where
    R: Resolution,
    C: dmac::channel::AnyChannel<Status = dmac::channel::Ready>,
    B: dmac::Buffer<Beat = <<SingleEnded<R> as CaptureType>::Sample as RawSample>::Count>,
{
    /// Create a single-ended software-triggered capture
    pub fn to_capture<I, A, P, BP, DT>(
        self,
        mut adc: Adc<I, A>,
        pos: P,
    ) -> Result<DmaSingleEndedCapture<N, I, A, P, BP, dmac::SpecificTransfer<DT>>, BuilderError>
    where
        I: AdcInstance,
        A: Accumulation<OutputResolution = R>,
        P: PosChannel<I>,
        BP: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst = B>,
        DT: dmac::AnyTransfer<Buf = BP, State = dmac::Ready<C::Id>>,
        AdcResultBuffer<I>: dmac::PeripheralBuffer,
    {
        let peripheral_buffer = adc.dma_buffer_take().ok_or(BuilderError::DmaBufferUnavailable)?;
        let dma_transfer = dmac::Transfer::new(
            self.dma_channel.into(),
            peripheral_buffer,
            self.dst_buffer,
            false,
            <AdcResultBuffer<I> as dmac::PeripheralBuffer>::TRIG_SRC,
            dmac::TriggerAction::Burst,
        )?;

        Ok(DmaSingleEndedCapture::<_, _, _, _, _, _>::from_channel(adc, pos, dma_transfer, self.oneshot))
    }
}

impl<R, EC, C, B, const N: usize> CaptureBuilder<R, SingleEnded<R>, EC, C, B, N>
where
    R: Resolution,
    EC: evsys::AnyChannel<Status = evsys::Asynchronous>,
    C: dmac::channel::AnyChannel<Status = dmac::channel::Ready>,
    B: dmac::Buffer<Beat = <<SingleEnded<R> as CaptureType>::Sample as RawSample>::Count>,
{
    /// Create a single-ended event triggered capture
    pub fn to_capture<I, A, P, BP, DT, E>(
        self,
        mut adc: Adc<I, A>,
        pos: P,
        evsys_controller: &mut evsys::EvsysController,
    ) -> Result<DmaSingleEndedCapture<N, I, A, P, BP, dmac::SpecificTransfer<DT>, E>, BuilderError>
    where
        I: AdcInstance,
        A: Accumulation<OutputResolution = R>,
        P: PosChannel<I>,
        BP: dmac::AnyBufferPair<Src = AdcResultBuffer<I>, Dst = B>,
        DT: dmac::AnyTransfer<Buf = BP, State = dmac::Ready<C::Id>>,
        E: evsys::AnyEvent<Channel = EC, User = evsys::User<I::StartEventId, EC::Id>>,
        AdcResultBuffer<I>: dmac::PeripheralBuffer,
    {
        let peripheral_buffer = adc.dma_buffer_take().ok_or(BuilderError::DmaBufferUnavailable)?;
        let dma_transfer = dmac::Transfer::new(
            self.dma_channel.into(),
            peripheral_buffer,
            self.dst_buffer,
            false,
            <AdcResultBuffer<I> as dmac::PeripheralBuffer>::TRIG_SRC,
            dmac::TriggerAction::Burst,
        )?;

        let user = evsys_controller
            .with_asynchronous_channel::<_, I::StartEventId>(&self.trigger_mode)?;
        let event = evsys::Event::new(self.trigger_mode, user);

        Ok(
            DmaSingleEndedCapture::<_, _, _, _, _, _, E>::from_channel(
                adc,
                pos,
                dma_transfer,
                event.into(),
                self.oneshot,
            )
        )
    }
}

impl<R: Resolution, T: CaptureType> Default for CaptureBuilder<R, T> {
    fn default() -> Self {
        Self {
            oneshot: false,
            trigger_mode: NoneT::default(),
            dma_channel: NoneT::default(),
            dst_buffer: NoneT::default(),
            _type: PhantomData,
            _resolution: PhantomData,
        }
    }
}
