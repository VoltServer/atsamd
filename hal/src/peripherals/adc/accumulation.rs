use atsamd_hal_macros::hal_cfg;

#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
use crate::pac::adc as adc0;

pub use adc0::ctrlb::Resselselect as ResolutionSelect;
pub use adc0::avgctrl::Samplenumselect as SampleCount;

use super::{Resolution, _16Bit, _15Bit, _14Bit, _13Bit, _12Bit, _10Bit, _8Bit};

use paste::paste;
use core::marker::PhantomData;
use crate::typelevel::Sealed;

#[repr(u8)]
pub enum DivisionFactor {
    _1 = 0,
    _2 = 1,
    _4 = 2,
    _8 = 3,
    _16 = 5,
    _32 = 6,
    _64 = 7,
}

//==============================================================================
// Accumulation
//==============================================================================
/// Type-level enum for various hardware accumulation strategies
pub trait Accumulation: Copy + Clone + PartialEq + Eq {
    type OutputResolution: Resolution;

    fn resselect(&self) -> ResolutionSelect;
    fn sample_count(&self) -> SampleCount;
    fn division_factor(&self) -> DivisionFactor;
}

pub type AccumulationResolution<A> = <A as Accumulation>::OutputResolution;

//==============================================================================
// "Single" sample accumulation (includes oversampling)
//==============================================================================
/// Type-level variant of [`Accumulation`] for "single" samples
///
/// "Single" accumulation strategy includes oversampling for acheiving
/// samples with higher resolutions than the 12-bit ADC natively supports
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Single<R: Resolution> {
    _res: PhantomData<R>,
}

impl<R: Resolution> Single<R> {
    pub const fn new() -> Self {
        Self {
            _res: PhantomData,
        }
    }
}

impl<R: Resolution> Single<R> {
    pub const fn _8_bit() -> Single<_8Bit> {
        Single::<_8Bit>::new()
    }

    pub const fn _10_bit() -> Single<_10Bit> {
        Single::<_10Bit>::new()
    }

    pub const fn _12_bit() -> Single<_12Bit> {
        Single::<_12Bit>::new()
    }

    pub const fn _13_bit() -> Single<_13Bit> {
        Single::<_13Bit>::new()
    }

    pub const fn _14_bit() -> Single<_14Bit> {
        Single::<_14Bit>::new()
    }

    pub const fn _15_bit() -> Single<_15Bit> {
        Single::<_15Bit>::new()
    }

    pub const fn _16_bit() -> Single<_16Bit> {
        Single::<_16Bit>::new()
    }
}

macro_rules! single_accumulation_impl {
    (
        $(
            $res:ident ($sample_count:path, $div_factor:path)
        ),+
        $(,)?
    ) => {
        paste! {
            $(
                impl Accumulation for Single<$res> {
                    type OutputResolution = $res;

                    fn resselect(&self) -> ResolutionSelect {
                        $res::SELECT
                    }

                    fn sample_count(&self) -> SampleCount {
                        $sample_count
                    }

                    fn division_factor(&self) -> DivisionFactor {
                        $div_factor
                    }
                }
            )+
        }
    };
}

single_accumulation_impl! {
    _8Bit (SampleCount::_1, DivisionFactor::_1),
    _10Bit (SampleCount::_1, DivisionFactor::_1),
    _12Bit (SampleCount::_1, DivisionFactor::_1),
    // Sample count and division factor settings for resolutions
    // over 12-bit are derived from Table 45-4 of the SAMD5x/E5x datasheet
    _13Bit (SampleCount::_4, DivisionFactor::_2),
    _14Bit (SampleCount::_16, DivisionFactor::_4),
    _15Bit (SampleCount::_64, DivisionFactor::_2),
    _16Bit (SampleCount::_256, DivisionFactor::_4),
}

//==============================================================================
// Hardware averaging
//==============================================================================
/// Type-level variant of [`Accumulation`] for hardware-averaged sampling
///
/// Hardware averaging accumulation strategy which averages up to 1024 samples
/// together. 
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Average {
    sample_count: SampleCount
}

impl Average {
    pub const fn new(sample_count: SampleCount) -> Self {
        Self {
            sample_count,
        }
    }
}

impl Accumulation for Average {
    type OutputResolution = _12Bit;

    fn resselect(&self) -> ResolutionSelect {
        match self.sample_count {
            SampleCount::_1 => ResolutionSelect::_12bit,
            _ => ResolutionSelect::_16bit,
        }
    }

    fn sample_count(&self) -> SampleCount {
        self.sample_count
    }

    fn division_factor(&self) -> DivisionFactor {
        match self.sample_count {
            SampleCount::_1 => DivisionFactor::_1,
            SampleCount::_2 => DivisionFactor::_2,
            SampleCount::_4 => DivisionFactor::_4,
            SampleCount::_8 => DivisionFactor::_8,
            SampleCount::_16 => DivisionFactor::_16,
            // when SAMPLENUM is greater than 16, the hardware begins to automatically
            // right-shift the result to prevent overflow, therefore we only need
            // a division factor of 16 for the remainder of the SampleCount variants
            // (see SAMD5x/E5x datasheet Table 45-3 for more detail)
            _ => DivisionFactor::_16,
        }
    }
}

//==============================================================================
// Hardware Summation
//==============================================================================
/// Type-level variant of [`Accumulation`] for hardware-summed sampling
///
/// Hardware summation accumulation strategy which can sum up to 1024 samples
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Summed<S: Samples> {
    _samples: PhantomData<S>,
}

impl<S: Samples> Accumulation for Summed<S> {
    type OutputResolution = S::OutputResolution;

    fn resselect(&self) -> ResolutionSelect {
        Self::OutputResolution::SELECT
    }

    fn sample_count(&self) -> SampleCount {
        S::COUNT
    }

    fn division_factor(&self) -> DivisionFactor {
        DivisionFactor::_1
    }
}

/// Type-level enum representing the number of summed samples used
/// for hardware-summed sampling.
pub trait Samples: Sealed + Copy + Clone + PartialEq + Eq {
    const COUNT: SampleCount;

    type OutputResolution: Resolution;
}

macro_rules! define_samples {
    (
        $(
            $sample:ident ($res:ident)
        ),+
        $(,)?
    ) => {
        paste! {
            $(
                #[doc = "Type-level variant of [`Samples`]"]
                #[derive(Copy, Clone, PartialEq, Eq)]
                pub enum [< $sample Samples >] {}

                impl Sealed for [< $sample Samples >] {}

                impl Samples for [< $sample Samples >] {
                    const COUNT: SampleCount = SampleCount::$sample;
                    type OutputResolution = $res;
                }
            )+
        }
    };
}

define_samples! {
    _1 (_12Bit),
    _2 (_13Bit),
    _4 (_14Bit),
    _8 (_15Bit),
    _16 (_16Bit),
    _32 (_16Bit),
    _64 (_16Bit),
    _128 (_16Bit),
    _256 (_16Bit),
    _512 (_16Bit),
    _1024 (_16Bit),
}
