use atsamd_hal_macros::hal_cfg;

#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
use crate::pac::adc as adc0;

use adc0::ctrlb::Resselselect as ResolutionSelect;

use crate::typelevel::Sealed;

/// Type-level enum for representing sample resolution
pub trait Resolution: Sealed + Copy + Clone + PartialEq + Eq + Default  {
    const BITS: u32;
    const SELECT: ResolutionSelect;
}

/// Type-level variant of [`Resolution`] for 16-bit ADC samples
///
/// # Note
/// This requires the use of hardware oversampling or summation to
/// achieve.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _16Bit {}

/// Type-level variant of [`Resolution`] for 15-bit ADC samples
///
/// # Note
/// This requires the use of hardware oversampling or summation to
/// achieve.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _15Bit {}

/// Type-level variant of [`Resolution`] for 14-bit ADC samples
///
/// # Note
/// This requires the use of hardware oversampling or summation to
/// achieve.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _14Bit {}

/// Type-level variant of [`Resolution`] for 13-bit ADC samples
///
/// # Note
/// This requires the use of hardware oversampling or summation to
/// achieve.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _13Bit {}

/// Type-level variant of [`Resolution`] for 12-bit ADC samples
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _12Bit {}

/// Type-level variant of [`Resolution`] for 10-bit ADC samples
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _10Bit {}

/// Type-level variant of [`Resolution`] for 8-bit ADC samples
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct _8Bit {}

impl Sealed for _16Bit {}
impl Sealed for _15Bit {}
impl Sealed for _14Bit {}
impl Sealed for _13Bit {}
impl Sealed for _12Bit {}
impl Sealed for _10Bit {}
impl Sealed for _8Bit {}

impl Resolution for _16Bit {
    const BITS: u32 = 16;
    const SELECT: ResolutionSelect = ResolutionSelect::_16bit;
}

impl Resolution for _15Bit {
    const BITS: u32 = 15;
    const SELECT: ResolutionSelect = ResolutionSelect::_16bit;
}

impl Resolution for _14Bit {
    const BITS: u32 = 14;
    const SELECT: ResolutionSelect = ResolutionSelect::_16bit;
}

impl Resolution for _13Bit {
    const BITS: u32 = 13;
    const SELECT: ResolutionSelect = ResolutionSelect::_16bit;
}

impl Resolution for _12Bit {
    const BITS: u32 = 12;
    const SELECT: ResolutionSelect = ResolutionSelect::_12bit;
}

impl Resolution for _10Bit {
    const BITS: u32 = 10;
    const SELECT: ResolutionSelect = ResolutionSelect::_10bit;
}

impl Resolution for _8Bit {
    const BITS: u32 = 8;
    const SELECT: ResolutionSelect = ResolutionSelect::_8bit;
}
