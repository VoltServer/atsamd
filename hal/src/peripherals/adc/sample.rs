use voltserver_hal::adc::{RawSample, UnsignedRawSample, SignedRawSample};
use core::marker::PhantomData;
use crate::typelevel::Sealed;
use super::resolution::Resolution;

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
    pub fn from_array<const N: usize>(raw_samples: &[u16; N], left_adj: bool) -> Option<[Self; N]> {
        let mut buffer = [Self::default(); N];

        for (i, raw_sample) in raw_samples.iter().enumerate() {
            let shift_amt = if left_adj {
                u16::BITS - R::BITS
            } else {
                0
            };

            let sample = Self::new(*raw_sample >> shift_amt);

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
    pub unsafe fn from_array_unchecked<const N: usize>(raw_samples: &[u16; N], left_adj: bool) -> [Self; N] {
        let mut buffer = [Self::default(); N];

        for (i, raw_sample) in raw_samples.iter().enumerate() {
            let shift_amt = if left_adj {
                u16::BITS - R::BITS
            } else {
                0
            };

            buffer[i] = unsafe { Self::new_unchecked(*raw_sample >> shift_amt) };
        }

        buffer
    }
}

impl <R: Resolution> RawSample for UnsignedSample<R> {
    const RESOLUTION: u32 = R::BITS;
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
    const RESOLUTION: u32 = R::BITS;
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
