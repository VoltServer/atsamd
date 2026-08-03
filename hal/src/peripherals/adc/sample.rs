use voltserver_hal::adc::{RawSample, UnsignedRawSample, SignedRawSample};
use core::marker::PhantomData;
use crate::typelevel::Sealed;

pub trait Resolution: Sealed + Copy {
    const RESOLUTION: u32;
}

#[derive(Copy, Clone)]
pub enum _12Bit {}
#[derive(Copy, Clone)]
pub enum _10Bit {}
#[derive(Copy, Clone)]
pub enum _8Bit {}

impl Sealed for _12Bit {}
impl Resolution for _12Bit {
    const RESOLUTION: u32 = 12;
}

impl Sealed for _10Bit {}
impl Resolution for _10Bit {
    const RESOLUTION: u32 = 10;
}

impl Sealed for _8Bit {}
impl Resolution for _8Bit {
    const RESOLUTION: u32 = 8;
}


#[derive(Copy, Clone)]
pub struct UnsignedSample<R: Resolution> {
    count: u16,
    _res: PhantomData<R>,
}

impl<R: Resolution> Default for UnsignedSample<R> {
    fn default() -> Self {
        Self {
            count: 0,
            _res: PhantomData,
        }
    }
}

impl<R: Resolution> UnsignedSample<R> {
    /// Creates an array of `UnsignedSample`s from an array of raw `u16`s
    /// Returns `None` if any of the samples cannot be represented at the specified
    /// resolution.
    pub fn from_array<const N: usize>(raw_samples: &[u16; N]) -> Option<[Self; N]> {
        let mut buffer = [Self::default(); N];

        for (i, raw_sample) in raw_samples.iter().enumerate() {
            let sample = Self::new(*raw_sample);

            if let Some(sample) = sample {
                buffer[i] = sample;
            } else {
                return None;
            }
        }

        Some(buffer)
    }

    /// Creates an array of `UnsignedSample`s from an array of raw `u16`s without checking
    /// that the input values are valid at the specified resolution.
    ///
    /// # Safety
    /// Caller must ensure that values contained in `raw_samples` are valid at the
    /// specified resolution.
    pub unsafe fn from_array_unchecked<const N: usize>(raw_samples: &[u16; N]) -> [Self; N] {
        let mut buffer = [Self::default(); N];

        for (i, raw_sample) in raw_samples.iter().enumerate() {
            buffer[i] = unsafe { Self::new_unchecked(*raw_sample) };
        }

        buffer
    }
}

impl <R: Resolution> RawSample for UnsignedSample<R> {
    const RESOLUTION: u32 = R::RESOLUTION;
    type Count = u16;

    fn new(count: Self::Count) -> Option<Self> {
        if count <= Self::max_count() {
            unsafe { Some(Self::new_unchecked(count)) }
        } else {
            None
        }
    }

    unsafe fn new_unchecked(count: Self::Count) -> Self {
        Self {
            count,
            _res: PhantomData,
        }
    }

    fn count(&self) -> Self::Count {
        self.count
    }
}

#[derive(Copy, Clone)]
pub struct SignedSample<R: Resolution> {
    count: i16,
    _res: PhantomData<R>,
}

impl<R: Resolution> RawSample for SignedSample<R> {
    const RESOLUTION: u32 = R::RESOLUTION;
    type Count = i16;

    fn new(count: Self::Count) -> Option<Self> {
        if count <= Self::max_count() {
            unsafe { Some(Self::new_unchecked(count)) }
        } else {
            None
        }
    }

    unsafe fn new_unchecked(count: Self::Count) -> Self {
        Self {
            count,
            _res: PhantomData,
        }
    }

    fn count(&self) -> Self::Count {
        self.count
    }
}


pub trait NumAccumulatedSamples {
    const FINAL_RESOLUTION: u32;
    const DIV_FACTOR: u32;
}
pub enum _1Sample {}
impl NumAccumulatedSamples for _1Sample {
    const FINAL_RESOLUTION: u32 = 12;
    const DIV_FACTOR: u32 = 0;
}
pub enum _2Samples {}
impl NumAccumulatedSamples for _2Samples {
    const FINAL_RESOLUTION: u32 = 13;
    const DIV_FACTOR: u32 = 0;
}
pub enum _4Samples {}
impl NumAccumulatedSamples for _4Samples {
    const FINAL_RESOLUTION: u32 = 14;
    const DIV_FACTOR: u32 = 0;
}
pub enum _8Samples {}
impl NumAccumulatedSamples for _8Samples {
    const FINAL_RESOLUTION: u32 = 15;
    const DIV_FACTOR: u32 = 0;
}
pub enum _16Samples {}
impl NumAccumulatedSamples for _16Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 0;
}
pub enum _32Samples {}
impl NumAccumulatedSamples for _32Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 2;
}
pub enum _64Samples {}
impl NumAccumulatedSamples for _64Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 4;
}
pub enum _128Samples {}
impl NumAccumulatedSamples for _128Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 8;
}
pub enum _256Samples {}
impl NumAccumulatedSamples for _256Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 16;
}
pub enum _512Samples {}
impl NumAccumulatedSamples for _512Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 32;
}
pub enum _1024Samples {}
impl NumAccumulatedSamples for _1024Samples {
    const FINAL_RESOLUTION: u32 = 16;
    const DIV_FACTOR: u32 = 64;
}

pub struct AccumulatedUnsignedSample<S: NumAccumulatedSamples>{
    count: u16,
    _samples: PhantomData<S>
}

impl<S: NumAccumulatedSamples> RawSample for AccumulatedUnsignedSample<S> {
    const RESOLUTION: u32 = S::FINAL_RESOLUTION;
    type Count = u16;

    fn new(count: Self::Count) -> Option<Self> {
        if count <= Self::max_count() {
            unsafe { Some(Self::new_unchecked(count)) }
        } else {
            None
        }
    }

    unsafe fn new_unchecked(count: Self::Count) -> Self {
        Self{
            count,
            _samples: PhantomData,
        }
    }

    fn count(&self) -> Self::Count {
        self.count
    }
}
