use voltserver_hal::{
    adc::{Capture, ReadyCapture, InProgressCapture, CompleteCapture, RawSample},
    dma::{ReadableDstBuffer},
};
use crate::adc::{Adc, AdcInstance, PosChannel, NegChannel, PosAdcPin, NegAdcPin, sample::{Resolution, SignedSample, UnsignedSample}};
use core::marker::PhantomData;
use crate::typelevel::Sealed;

use crate::dmac;
use dmac::transfer::State as TransferState;
use dmac::transfer::TransferChannelId;
use dmac::BufferPairBeat;

pub struct SingleEndedCapture<const N: usize, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    adc: Adc<I>,
    dma_transfer: T,
    _pos: PhantomData<P>,
    _res: PhantomData<R>,
}

impl<const N: usize, I, P, R, B, T> SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count> + ReadableDstBuffer<BufferPairBeat<B>>>,
    T: dmac::ReadyTransfer<Buf = B>,
{
    fn from_channel(adc: Adc<I>, _pos: P, dma_transfer: T) -> Self {
        Self {
            adc,
            dma_transfer,
            _pos: PhantomData,
            _res: PhantomData,
        }
    }

    fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I>, _pin: Pin, dma_transfer: T) -> Self {
        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel(), dma_transfer)
    }
}

impl<const N: usize, I, P, R, B, T> Capture for SingleEndedCapture<N, I, P, R, B, T>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    B: dmac::AnyBufferPair<Src = Adc<I>, Dst: dmac::Buffer<Beat = <UnsignedSample<R> as RawSample>::Count> + ReadableDstBuffer<BufferPairBeat<B>>>,
    T: dmac::AnyTransfer<Buf = B>,
{
    type Error = crate::adc::Error;
    type Sample = UnsignedSample<R>;
    type Output = [Self::Sample; N];
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

    fn start(self) -> Result<Self::InProgress, Self::Error> {
        //TODO: configure ADC
        //TODO: start DMA

        todo!()
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
        self.dma_transfer.software_trigger();
        Ok(())
    }

    fn is_complete(&mut self) -> Result<bool, Self::Error> {
        Ok(self.dma_transfer.is_complete())
    }

    fn wait(&mut self) -> Result<(), Self::Error> {
        while !self.dma_transfer.is_complete() {}
        Ok(())
    }

    fn stop(self) -> Result<Self::Complete, Self::Error> {
        Ok(Self::Complete {
            adc: self.adc,
            dma_transfer: self.dma_transfer.stop(),
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
