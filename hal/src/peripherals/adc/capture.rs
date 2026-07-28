use voltserver_hal::adc::{Capture, ReadyCapture, InProgressCapture, CompleteCapture, RawSample};
use super::{Adc, AdcInstance, PosChannel, NegChannel, PosAdcPin, NegAdcPin, sample::{Resolution, SignedSample, UnsignedSample}};
use core::marker::PhantomData;
use crate::typelevel::Sealed;

pub trait State: Sealed {}
pub enum Ready {}
pub enum InProgress {}
pub enum Complete {}

impl Sealed for Ready {}
impl Sealed for InProgress {}
impl Sealed for Complete {}

impl State for Ready {}
impl State for InProgress {}
impl State for Complete {}

pub struct SingleEndedCapture<const N: usize, I, P, R, S>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    S: State,
{
    adc: Adc<I>,
    buffer: [UnsignedSample<R>; N],
    _pos: PhantomData<P>,
    _state: PhantomData<S>,
}

impl<const N: usize, I, P, R> SingleEndedCapture<N, I, P, R, Ready>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
{
    fn from_channel(adc: Adc<I>, _pos: P) -> Self {
        Self {
            adc,
            buffer: unsafe { [<UnsignedSample<R> as RawSample>::new_unchecked(0) ;N] },
            _pos: PhantomData,
            _state: PhantomData,
        }
    }

    fn from_pin<Pin: PosAdcPin<I, Channel = P>>(adc: Adc<I>, _pin: Pin) -> Self {
        Self::from_channel(adc, <Pin as PosAdcPin<I>>::Channel::get_channel())
    }
}

impl<const N: usize, I, P, R, S> Capture for SingleEndedCapture<N, I, P, R, S>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
    S: State,
{
    type Error = super::Error;
    type Sample = UnsignedSample<R>;
    type Output = [Self::Sample; N];
}

impl<const N: usize, I, P, R> ReadyCapture for SingleEndedCapture<N, I, P, R, Ready>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
{
    type InProgress = SingleEndedCapture<N, I, P, R, InProgress>;

    fn start(self) -> Result<Self::InProgress, Self::Error> {
        //TODO: create & start DMA transfer
        todo!()
    }
}

impl<const N: usize, I, P, R> InProgressCapture for SingleEndedCapture<N, I, P, R, InProgress>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
{
    type Ready = SingleEndedCapture<N, I, P, R, Ready>;
    type Complete = SingleEndedCapture<N, I, P, R, Complete>;

    fn trigger(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn is_complete(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn wait(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn stop(self) -> Result<Self::Complete, Self::Error> {
        todo!()
    }
}

impl<const N: usize, I, P, R> CompleteCapture for SingleEndedCapture<N, I, P, R, Complete>
where
    I: AdcInstance,
    P: PosChannel<I>,
    R: Resolution,
{
    type Ready = SingleEndedCapture<N, I, P, R, Ready>;

    fn reset(self) -> Self::Ready {
        todo!()
    }

    fn convert(self) -> Result<(Self::Ready, Self::Output), Self::Error> {
        todo!()
    }
}

pub struct DifferentialCapture<const N: usize, I, PC, NC, R, S>
where
    I: AdcInstance,
    PC: PosChannel<I>,
    NC: NegChannel<I>,
    R: Resolution,
    S: State,
{
    adc: Adc<I>,
    buffer: [SignedSample<R>; N],
    _pos: PhantomData<PC>,
    _neg: PhantomData<NC>,
    _state: PhantomData<S>,
}

impl<const N: usize, I, PC, NC, R, S> Capture for DifferentialCapture<N, I, PC, NC, R, S>
where
    I: AdcInstance,
    PC: PosChannel<I>,
    NC: NegChannel<I>,
    R: Resolution,
    S: State,
{
    type Error = super::Error;
    type Sample = SignedSample<R>;
    type Output = [Self::Sample; N];
}

impl<const N: usize, I, PC, NC, R> ReadyCapture for DifferentialCapture<N, I, PC, NC, R, Ready>
where
    I: AdcInstance,
    PC: PosChannel<I>,
    NC: NegChannel<I>,
    R: Resolution,
{
    type InProgress = DifferentialCapture<N, I, PC, NC, R, InProgress>;

    fn start(self) -> Result<Self::InProgress, Self::Error> {
        todo!()
    }
}

impl<const N: usize, I, PC, NC, R> InProgressCapture for DifferentialCapture<N, I, PC, NC, R, InProgress>
where
    I: AdcInstance,
    PC: PosChannel<I>,
    NC: NegChannel<I>,
    R: Resolution,
{
    type Ready = DifferentialCapture<N, I, PC, NC, R, Ready>;
    type Complete = DifferentialCapture<N, I, PC, NC, R, Complete>;

    fn trigger(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn is_complete(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn wait(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn stop(self) -> Result<Self::Complete, Self::Error> {
        todo!()
    }
}

impl<const N: usize, I, PC, NC, R> CompleteCapture for DifferentialCapture<N, I, PC, NC, R, Complete>
where
    I: AdcInstance,
    PC: PosChannel<I>,
    NC: NegChannel<I>,
    R: Resolution,
{
    type Ready = DifferentialCapture<N, I, PC, NC, R, Ready>;

    fn reset(self) -> Self::Ready {
        todo!()
    }

    fn convert(self) -> Result<(Self::Ready, Self::Output), Self::Error> {
        todo!()
    }
}
