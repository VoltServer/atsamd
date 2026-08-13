//! # Event System
//!
//! This library provides a type-safe API with compile-time gaurentees that
//! the event channel is properly configured and compatible with the event user.
//!
//! # [`Event`]s
//! `Event`s represents a complete event path from source to consumer through
//! the EVSYS peripheral. The [`Event`] owns its [`Channel`] and [`UserMux`]
//! objects which contain type-encoded information relating to the physical
//! EVSYS channel and EVSYS user multiplexer.
//!
//! Peripherals which can consume events through EVSYS may require an [`Event`]
//! be passed in during initialization or configuration as a compile-time proof
//! that both the EVSYS channel and EVSYS user multiplexer has been properly
//! configured.
//!
//! ## [`Channel`]s
//! `Channel`s encapsulate the EVSYS channel registers which allow configuration
//! of all channel-specific operation on a per-channel basis. Individual
//! channels are accessed through [`EvsysController::split()`].
//!
//! ## [`UserMux`]s
//! `UserMux`s represent the EVSYS `USER` register which routes events generated
//! from a channel to a specific event consumer (usually another peripheral).
//! Each `UserMux` is a singleton, with access managed through the consumer
//! peripheral and [`EvsysController`]. On initialization of the
//! `EvsysController`, all `UserMux`s are stored internally. A peripherals
//! `UserMux` may be requested through the trait methods
//! [`AsyncUser::with_asynchronous_channel`],
//! [`SyncUser::with_synchronous_channel`],
//! or [`SyncUser::with_resynchronized_channel`]. Alternatively, unsafe access
//! is provided through [`User::with_channel_unchecked`] which does not
//! guarentee that the channel is properly configured before configuring the
//! `UserMux`.
//!
//! # Controller
//!
//! # Example
//! ```
//! todo!()
//! ```

use atsamd_hal_macros::hal_cfg;

pub use channel::*;
pub use event::*;
pub use evsys_controller::*;
pub use user::*;

pub mod channel;
pub mod event;
pub mod evsys_controller;
pub mod user;

pub enum Error {
    /// The EVSYS channel is already in use
    ChannelInUse,
    /// The EVSYS user is already in use
    UserInUse,
}

#[hal_cfg("dmac-d5x")]
macro_rules! with_num_evsys_channels {
    ($some_macro:ident) => {
        $some_macro! {32}
    };
}
pub(super) use with_num_evsys_channels;

#[hal_cfg("dmac-d5x")]
macro_rules! with_num_evsys_synchronous_channels {
    ($some_macro:ident) => {
        $some_macro! {12}
    };
}
pub(super) use with_num_evsys_synchronous_channels;
