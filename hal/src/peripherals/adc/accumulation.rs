use atsamd_hal_macros::hal_cfg;
use voltserver_hal::adc::{Resolution, _8Bit, _10Bit, _12Bit, _13Bit, _14Bit, _15Bit, _16Bit};

#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
use crate::pac::adc as adc0;

pub use adc0::ctrlb::Resselselect as ResolutionSelect;
pub use adc0::avgctrl::Samplenumselect as SampleCount;

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
pub trait Accumulation: Clone + Copy {
    type Resolution: voltserver_hal::adc::Resolution;

    fn resselect(&self) -> ResolutionSelect;
    fn sample_count(&self) -> SampleCount;
    fn division_factor(&self) -> DivisionFactor;
}
pub type AccumulationResolution<A> = <A as Accumulation>::Resolution;

//==============================================================================
// NativeResolution
//==============================================================================
/// Trait for resolutions that are natively supported by the ADC
pub trait NativeResolution: voltserver_hal::adc::Resolution + Sealed {
    const SEL: ResolutionSelect;
}

macro_rules! native_resolution_impl {
    (
        $(
            $res:ident ($select:path)
        ),+
        $(,)?
    ) => {
        $(
            impl Sealed for $res {}
            impl NativeResolution for $res {
                const SEL: ResolutionSelect = $select;
            }
        )+
    };
}

native_resolution_impl!{
    _8Bit (ResolutionSelect::_8bit),
    _10Bit (ResolutionSelect::_10bit),
    _12Bit (ResolutionSelect::_12bit),
}


//==============================================================================
// Single 8, 10, or 12-bit sampling
//==============================================================================
/// Type-level variant of [`Accumulation`] for native resolution samples
///
#[derive(Clone, Copy)]
pub struct Single<R: NativeResolution> {
    _res: PhantomData<R>,
}

impl<R: NativeResolution> Single<R> {
    const fn new() -> Self {
        Self {
            _res: PhantomData,
        }
    }
}

macro_rules! single_accumulation_impl {
    (
        $(
            $res:ident
        ),+
        $(,)?
    ) => {
        paste! {
            impl<R: NativeResolution> Single<R> {
                $(
                    pub const fn [< $res:snake >]() -> Single<$res> {
                        Single::new()
                    }
                )+
            }
            $(
                impl Accumulation for Single<$res> {
                    type Resolution = $res;

                    fn resselect(&self) -> ResolutionSelect {
                        $res::SEL
                    }

                    fn sample_count(&self) -> SampleCount {
                        SampleCount::_1
                    }

                    fn division_factor(&self) -> DivisionFactor {
                        DivisionFactor::_1
                    }
                }
            )+
        }
    };
}

single_accumulation_impl! {
    _8Bit,
    _10Bit,
    _12Bit,
}

//==============================================================================
// Hardware Oversampling (up to 16-bit)
//==============================================================================
/// Type-level variant of [`Accumulation`] for >12-bit samples
#[derive(Clone, Copy)]
pub struct Oversample<R: Resolution> {
    _res: PhantomData<R>,
}

impl<R: Resolution> Oversample<R> {
    const fn new() -> Self {
        Self {
            _res: PhantomData,
        }
    }
}

macro_rules! oversample_accumulation_impl {
    (
        $(
            $res:ident ($sample_count:path, $div_factor:path)
        ),+
        $(,)?
    ) => {
        paste! {
            impl<R: Resolution> Oversample<R> {
                $(
                    pub const fn [< $res:snake >]() -> Oversample<$res> {
                        Oversample::new()
                    }
                )+
            }
            $(
                impl Accumulation for Oversample<$res> {
                    type Resolution = $res;

                    fn resselect(&self) -> ResolutionSelect {
                        ResolutionSelect::_12bit
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

oversample_accumulation_impl! {
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
//#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Clone, Copy)]
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
    type Resolution = _12Bit;

    fn resselect(&self) -> ResolutionSelect {
        ResolutionSelect::_12bit
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
//#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Clone, Copy)]
pub struct Summed<S: Samples> {
    _samples: PhantomData<S>,
}

impl<S: Samples> Accumulation for Summed<S> {
    type Resolution = S::Resolution;

    fn resselect(&self) -> ResolutionSelect {
        //Self::Resolution::SELECT
        todo!()
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

    type Resolution: Resolution;
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
                    type Resolution = $res;
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















////==============================================================================
//// "Single" sample accumulation
////==============================================================================
///// Type-level variant of [`Accumulation`] for "single" samples
/////
//#[derive(Copy, Clone, PartialEq, Eq, Default)]
//pub struct Single<const RES: u32 = 0> {}
//
//impl<const RES: u32> Single<RES> {
//    const fn new() -> Self {
//        Self {}
//    }
//}
//
//impl Single {
//    pub const fn _8_bit() -> Single<8> {
//        Single::<8>::new()
//    }
//
//    pub const fn _10_bit() -> Single<10> {
//        Single::<10>::new()
//    }
//
//    pub const fn _12_bit() -> Single<12> {
//        Single::<10>::new()
//    }
//}
//
//impl Accumulation for Single<8> {
//    const RES: u32 = 8;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_8bit
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_1
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_1
//    }
//}
//
//impl Accumulation for Single<10> {
//    const RES: u32 = 8;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_10bit
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_1
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_1
//    }
//}
//
//impl Accumulation for Single<12> {
//    const RES: u32 = 8;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_12bit
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_1
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_1
//    }
//}
//
////==============================================================================
//// Oversampling sample accumulation
////==============================================================================
//pub struct Oversample<const RES: u32 = 0> {}
//
//impl<const RES: u32> Oversample<RES> {
//    const fn new() -> Self {
//        Self {}
//    }
//}
//
//impl Oversample {
//    pub const fn _13_bit() -> Oversample<13> {
//        Oversample::<13>::new()
//    }
//
//    pub const fn _14_bit() -> Oversample<14> {
//        Oversample::<14>::new()
//    }
//
//    pub const fn _15_bit() -> Oversample<15> {
//        Oversample::<15>::new()
//    }
//
//    pub const fn _16_bit() -> Oversample<16> {
//        Oversample::<16>::new()
//    }
//}
//
//impl Accumulation for Oversample<13> {
//    const RES: u32 = 13;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_12bit;
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_4
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_2
//    }
//}
//
//impl Accumulation for Oversample<14> {
//    const RES: u32 = 14;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_12bit;
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_16
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_4
//    }
//}
//
//impl Accumulation for Oversample<15> {
//    const RES: u32 = 15;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_12bit;
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_64
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_2
//    }
//}
//
//impl Accumulation for Oversample<16> {
//    const RES: u32 = 16;
//
//    fn resselect(&self) -> ResolutionSelect {
//        ResolutionSelect::_12bit;
//    }
//
//    fn sample_count(&self) -> SampleCount {
//        SampleCount::_256
//    }
//
//    fn division_factor(&self) -> DivisionFactor {
//        DivisionFactor::_4
//    }
//}



