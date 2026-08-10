//! # Abstractions over individual EVSYS channels
//!
//! # Initializing
//!
//! Channels may be initalized through [`Channel::asynchronous`],
//! [`Channel::synchronous`] or [`Channel::resyncrhonized`] depending on
//! which modes of operation the channel supports. Once intialized with one
//! of the three operating modes, the channel will begin generating events
//! from the configured event source (via [`Channel::event_source]`).
//! Interrupts for the channel may be enabled/disabled after initialization.
//!
//! # Channel status
//! The channel status represents the four possible states that a channel can
//! be in: [`Uninitialized`], [`Asynchronous`], [`Synchronous`] or
//! [`Resynchronized`]. Only certain channels may support the [`Asynchronous`]
//! and [`Synchronous`] modes.
//!
#![allow(unused_braces)]

use atsamd_hal_macros::hal_cfg;

use core::marker::PhantomData;
use crate::typelevel::{Sealed, Is};
use super::evsys_controller::{ChId, SynchronusCh};
use modular_bitfield::prelude::*;
use crate::clock::v2::pclk;

mod reg;
use reg::RegisterBlock;

#[hal_cfg("evsys-d5x")]
pub use crate::pac::evsys::channels::channel::{
    Evgenselect as EventSource,
    Edgselselect as EdgeType,
};

//==============================================================================
// Channel Status
//==============================================================================
pub trait Status: Sealed {}

/// Uninitialized channel
pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}

/// Channel configured for synchronous operation
pub enum Synchronous {}
impl Sealed for Synchronous {}
impl Status for Synchronous {}

/// Channel configured for resynchronized operation
pub enum Resynchronized {}
impl Sealed for Resynchronized {}
impl Status for Resynchronized {}

/// Channel configured for asynchronous operation
pub enum Asynchronous {}
impl Sealed for Asynchronous {}
impl Status for Asynchronous {}

//==============================================================================
// AnyChannel
//==============================================================================
pub trait AnyChannel: Sealed + Is<Type = SpecificChannel<Self>> {
    type Status: Status;
    type Id: ChId;
}

pub type SpecificChannel<C> = Channel<<C as AnyChannel>::Id, <C as AnyChannel>::Status>;

pub type ChannelStatus<C> = <C as AnyChannel>::Status;
pub type ChannelId<C> = <C as AnyChannel>::Id;

impl<Id, S> Sealed for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
}

impl<Id, S> AnyChannel for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    type Id = Id;
    type Status = S;
}

impl<Id, S> AsRef<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, S> AsMut<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Channel
//==============================================================================

/// Event channel, allows configuration of EVSYS channels.
/// The EVSYS hardware channel is disabled when the corresponding [`Channel`] is
/// dropped.
pub struct Channel<Id: ChId, S: Status> {
    regs: RegisterBlock<Id>,
    _status: PhantomData<S>,
}

#[inline]
pub(super) fn new_chan<Id: ChId>(_id: PhantomData<Id>) -> Channel<Id, Uninitialized> {
    Channel {
        regs: RegisterBlock::new(_id),
        _status: PhantomData,
    }
}

/// Methods for all [`Channel`]s
impl<Id: ChId, S: Status> Channel<Id, S> {
    /// Configure channel to use an asynchronous path for event propagation.
    ///
    /// All EVSYS channels support asynchronous propagation without need to
    /// configure the corresponding peripheral clock.
    #[inline]
    pub fn asynchronous(mut self) -> Channel<Id, Asynchronous> {
        self.regs.channel.modify(|_, w| w.path().asynchronous());

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    /// Configure channel event source
    #[inline]
    pub fn event_source(&mut self, source: EventSource) {
        self.regs.channel.modify(|_, w| w.evgen().variant(source));
    }

    /// Selectively enable interrupts
    #[inline]
    pub fn enable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenset
            .write(|w| unsafe { w.bits(flags.into()) });
        todo!()
    }

    /// Selectively disable interrupts
    #[inline]
    pub fn disable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenclr
            .write(|w| unsafe { w.bits(flags.into()) });
    }

    /// Check the specified `flags`, clear then return any that were set
    #[inline]
    pub fn check_and_clear_interrupts(&mut self, flags: InterruptFlags) -> InterruptFlags {
        let mut cleared = 0;
        self.regs
            .chintflag
            .modify(|r, w| {
                cleared = r.bits() & flags.into_bytes()[0];
                unsafe { w.bits(cleared) }
            });

        InterruptFlags::from_bytes([cleared])
    }

    /// Trigger an event for this channel via software
    #[inline]
    pub fn software_trigger(&mut self) {
        self.regs.swevt.set_bit();
    }

    /// Disable this channel, stopping future events from propagating and transforming
    /// the [`Channel`] back to its `Uninitialized` state.
    #[inline]
    pub fn disable(mut self) -> Channel<Id, Uninitialized> {
        self.regs
            .channel
            .write(|w| w.evgen().none());

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }
}

/// Methods for [`Channel`]s which support synchronus/resynchronized operation
impl<Id: SynchronusCh, S: Status> Channel<Id, S> {
    // TODO: Ideally, the Channel struct would take ownership of the Pclk type
    // when configured for use as a synchronous or resynchronized path. However,
    // since clock::v2 is not implemented for all chips yet, the
    // generics for the Channel type would be different between chip families,
    // leading to massive and unnecessary code duplication. In the meantime,
    // we use a "lite" variation of the typelevel guarantees laid out by the
    // clock::v2 module, meaning that we can guarantee that the clocks are enabled
    // at the time of creation of the channel struct; however we can't guarantee
    // that the clock will stay enabled for the duration of its lifetime.

    /// Configure the channel to use a synchronous path for event propagation.
    /// Requires the peripheral clock associated with this channel to be enabled.
    ///
    /// This method takes a reference to a [`Pclk`] as an argument to form a compile-time
    /// guarentee that the clock corresponding to this channel has been configured and
    /// enabled. In the future this will likely change to taking full ownership of it;
    /// in the meantime, the caller must ensure that the PCLK is enabled for the
    /// [`Channel`]s lifetime.
    #[inline]
    pub fn synchronous<PS: pclk::PclkSourceId>(
        mut self,
        _clk: &pclk::Pclk<Id, PS>,
        edge_type: EdgeType,
    ) -> Channel<Id, Synchronous> {
        self.regs.channel.modify(|_, w| {
            w.path().synchronous();
            w.edgsel().variant(edge_type)
        });

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    /// Configure the channel to use a resynchronized path for event propagation.
    /// Requires the peripheral clock associated with this channel to be enabled.
    ///
    /// This method takes a reference to a [`Pclk`] as an argument to form a compile-time
    /// guarentee that the clock corresponding to this channel has been configured and
    /// enabled. In the future this will likely change to taking full ownership of it;
    /// in the meantime, the caller must ensure that the PCLK is enabled for the
    /// [`Channel`]s lifetime.
    #[inline]
    pub fn resynchronized<PS: pclk::PclkSourceId>(
        mut self,
        _clk: &pclk::Pclk<Id, PS>,
        edge_type: EdgeType,
    ) -> Channel<Id, Resynchronized> {
        self.regs
            .channel
            .modify(|_, w| {
                w.path().resynchronized();
                w.edgsel().variant(edge_type)
            });

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    /// Configure the behavior of this channel when the processor enters standby
    #[inline]
    pub fn run_standby(&mut self, yes: bool) {
        self.regs
            .channel
            .modify(|_, w| {
                w.runstdby().variant(yes)
            });
    }

    /// Configure the behavior of this channel with respect to when the peripheral
    /// clock is requested.
    #[inline]
    pub fn on_demand(&mut self, yes: bool) {
        self.regs
            .channel
            .modify(|_, w| {
                w.ondemand().variant(yes)
            });
    }
}

#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
pub struct InterruptFlags {
    /// Channel overrun
    pub ovr: bool,
    /// Channel event detected
    pub evd: bool,
    #[skip]
    _reserved: B6,
}
