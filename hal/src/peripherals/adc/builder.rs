use atsamd_hal_macros::hal_cfg;

#[hal_cfg("adc-d5x")]
use crate::pac::adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
use crate::pac::adc as adc0;

#[hal_cfg(any("adc-d21", "adc-d11"))]
pub use adc0::ctrlb::Prescalerselect as Prescaler;

#[hal_cfg("adc-d5x")]
pub use adc0::ctrla::Prescalerselect as Prescaler;

pub use adc0::avgctrl::Samplenumselect as SampleCount;
pub use adc0::ctrlb::Resselselect as ResolutionSelect;
pub use adc0::refctrl::Refselselect as Reference;

use super::{Adc, AdcInstance, Accumulation, Resolution, AccumulationResolution};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BuilderError {
    /// Clock divider missing
    MissingClockDiv,
    /// Samples per clock missing
    MissingSampleClocks,
    /// Vref missing
    MissingVref,
    AdcError(super::Error),
}

impl From<super::Error> for BuilderError {
    fn from(value: super::Error) -> Self {
        Self::AdcError(value)
    }
}

/// # ADC Configuration Builder
///
/// This structure provides configuration of multiple factors which affect the
/// ADC's sampling characteristics.
///
/// The ADC clock is driven by the peripheral clock divided with a divider
/// selected via [AdcBuilder::with_clock_divider()].
///
/// A sample is taken over a number of ADC clock cycles configured by
/// [AdcBuilder::with_clock_cycles_per_sample()], and then transmitted to the
/// ADC register 1 clock cycle per bit of resolution - resolution is determined
/// by the accumulation mode selected by [AdcBuilder::new()].
///
/// The ADC can be configured to combine multiple readings in either an average
/// or summed mode (See [Accumulation]).
///
/// The formula for calculating Sample rate (SPS) is shown below, and
/// implemented in a helper method [AdcBuilder::calculate_sps()]:
///
/// ## For single sample
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (sample_clock_cycles + bit_width)
/// ```
/// ## For multiple samples 'n' (Averaging or Summed)
/// ```
/// SPS = (GCLK_ADC / clk_divider) / (n * (sample_clock_cycles + 12))
/// ```
#[derive(Copy, Clone)]
pub struct AdcBuilder<A: Accumulation> {
    pub clk_divider: Option<Prescaler>,
    pub sample_clock_cycles: Option<u8>,
    pub accumulation: A,
    pub vref: Option<Reference>,
    pub offset_compensation: Option<bool>,
    pub reference_compensation: Option<bool>,
    pub auto_left_adjust: Option<bool>,
    pub auto_rail_to_rail: Option<bool>,
}

/// Version of [AdcBuilder] without any optional settings.
/// [AdcBuilder] is converted to this when passed to the ADC
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct AdcSettings<A: Accumulation> {
    pub clk_divider: Prescaler,
    pub sample_clock_cycles: u8,
    pub accumulation: A,
    pub vref: Reference,
    pub offset_compensation: bool,
    pub reference_compensation: bool,
    pub auto_left_adjust: bool,
    pub auto_rail_to_rail: bool,
}

impl<A: Accumulation> AdcBuilder<A> {
    /// Create a new settings builder
    pub fn new(accumulation_method: A) -> Self {
        Self {
            clk_divider: None,
            sample_clock_cycles: None,
            accumulation: accumulation_method,
            vref: None,
            offset_compensation: None,
            reference_compensation: None,
            auto_left_adjust: None,
            auto_rail_to_rail: None,
        }
    }

    pub(crate) fn check_params(&self) -> Result<(), BuilderError> {
        self.clk_divider.ok_or(BuilderError::MissingClockDiv)?;
        self.sample_clock_cycles
            .ok_or(BuilderError::MissingSampleClocks)?;
        self.vref.ok_or(BuilderError::MissingVref)?;
        Ok(())
    }

    pub(crate) fn to_settings(self) -> Result<AdcSettings<A>, BuilderError> {
        self.check_params()?;
        Ok(AdcSettings {
            clk_divider: self.clk_divider.unwrap(),
            sample_clock_cycles: self.sample_clock_cycles.unwrap(),
            accumulation: self.accumulation,
            vref: self.vref.unwrap(),
            offset_compensation: self.offset_compensation.unwrap_or(false),
            reference_compensation: self.reference_compensation.unwrap_or(false),
            auto_left_adjust: self.auto_left_adjust.unwrap_or(false),
            auto_rail_to_rail: self.auto_rail_to_rail.unwrap_or(false),
        })
    }

    /// This setting adjusts the ADC clock frequency by dividing the input clock
    /// for the ADC.
    ///
    /// ## Example:
    /// * Input clock 48MHz, div 32 => ADC Clock is 1.5MHz
    pub fn with_clock_divider(mut self, div: Prescaler) -> Self {
        self.clk_divider = Some(div);
        self
    }

    /// Sets the ADC reference voltage source
    pub fn with_vref(mut self, reference: Reference) -> Self {
        self.vref = Some(reference);
        self
    }

    /// Sets the number of ADC clock cycles taken to sample a single sample. The
    /// higher this number, the longer it will take the ADC to sample each
    /// sample. Smaller values will make the ADC perform more samples per
    /// second, but there may be more noise in each sample leading to erratic
    /// values.
    ///
    /// ## Safety
    /// * This function clamps input value between 1 and 63, to conform to the
    ///   ADC registers min and max values.
    pub fn with_clock_cycles_per_sample(mut self, num: u8) -> Self {
        self.sample_clock_cycles = Some(num.clamp(1, 63)); // Clamp in range
        self
    }

    /// Returns a calculated sample rate based on the settings used
    pub fn calculate_sps(&self, clock_freq: u32) -> Result<u32, BuilderError> {
        self.check_params()?;

        let div = self.clk_divider.unwrap() as u32;
        let adc_clk_freq = clock_freq / div;
        let bit_width = AccumulationResolution::<A>::BITS;

        let mut clocks_per_sample = self.sample_clock_cycles.unwrap() as u32 + bit_width;

        //let samples = self.accumulation.samples();
        let samples = 2u32.pow(self.accumulation.sample_count() as u8 as u32);
        clocks_per_sample *= samples as u32;
        Ok(adc_clk_freq / clocks_per_sample)
    }

    /// Configure the ADC offset compensation
    ///
    /// ## Important
    /// * Enabling offset compesation forces the clock cycles per sample to be 4
    ///   GCLK cycles, any change to the cycles per sample via
    ///   [`Self::with_clock_cycles_per_sample()`] will be ignored.
    #[hal_cfg("adc-d5x")]
    pub fn enable_offset_compensation(mut self, enable: bool) -> Self {
        self.offset_compensation = Some(enable);
        self
    }

    /// Configure the ADC reference compensation
    pub fn enable_reference_compensation(mut self, enable: bool) -> Self {
        self.reference_compensation = Some(enable);
        self
    }

    /// Enables automatic left-adjustment when measuring differential inputs.
    /// This allows use of the ADC summation or averaging hardware with
    /// negative result values. Results are automatically right-shifted back
    /// appropriately.
    pub fn enable_auto_left_adjust(mut self, enable: bool) -> Self {
        self.auto_left_adjust = Some(enable);
        self
    }

    /// Automatically enables rail-to-rail operation when measuring a
    /// differential input. This relaxes common-mode input requirements on
    /// differential inputs and allows measurments closer to supply rails.
    ///
    /// ## Important
    /// * Enabling auto rail-to-rail incurs a slight runtime performance hit as
    ///   the CTRLA.R2R bit is enable-protected, meaning the ADC must be shut
    ///   down and re-enabled to enable/disable rail-to-rail mode.
    #[hal_cfg("adc-d5x")]
    pub fn enable_auto_rail_to_rail(mut self, enable: bool) -> Self {
        self.auto_rail_to_rail = Some(enable);
        self
    }

    /// Turn the builder into an ADC
    #[hal_cfg("adc-d5x")]
    #[inline]
    pub fn enable<I: AdcInstance, PS: crate::clock::v2::pclk::PclkSourceId>(
        self,
        adc: I::Instance,
        clk: crate::clock::v2::apb::ApbClk<I::ClockId>,
        pclk: &crate::clock::v2::pclk::Pclk<I::ClockId, PS>,
    ) -> Result<Adc<I, A>, BuilderError> {
        let settings = self.to_settings()?;
        Adc::new(adc, settings, clk, pclk).map_err(|e| e.into())
    }

    #[hal_cfg(any("adc-d11", "adc-d21"))]
    #[inline]
    pub fn enable<I: AdcInstance>(
        self,
        adc: I::Instance,
        pm: &mut crate::pac::Pm,
        clock: &crate::clock::AdcClock,
    ) -> Result<Adc<I, A>, BuilderError> {
        let settings = self.to_settings()?;
        Adc::new(adc, settings, pm, clock).map_err(|e| e.into())
    }
}
