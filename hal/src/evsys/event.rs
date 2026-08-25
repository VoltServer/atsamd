//! # Event Abstractions

use super::{
    Error,
    channel::{AnyChannel, ChannelId},
    user::{AnyUser, UserUid},
};
use crate::typelevel::{Is, Sealed};

//==============================================================================
// AnyEvent
//==============================================================================
/// Type-level abstraction over [`Event`]
pub trait AnyEvent: Sealed + Is<Type = SpecificEvent<Self>> {
    type Channel: AnyChannel;
    type User: AnyUser<ChId = EventChannelId<Self>>;

    fn channel_error(&mut self) -> Result<(), Error>;
    fn clear_channel_errors(&mut self);
}

pub type SpecificEvent<E> = Event<<E as AnyEvent>::Channel, <E as AnyEvent>::User>;

pub type EventChannel<E> = <E as AnyEvent>::Channel;
pub type EventChannelId<E> = ChannelId<EventChannel<E>>;
pub type EventUser<E> = <E as AnyEvent>::User;
pub type EventUserId<E> = UserUid<EventUser<E>>;

impl<C, U> Sealed for Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
}

impl<C, U> AnyEvent for Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
    type Channel = C;
    type User = U;

    fn channel_error(&mut self) -> Result<(), Error> {
        self.channel_error()
    }

    fn clear_channel_errors(&mut self) {
        self.clear_channel_errors();
    }
}

impl<C, U> AsRef<Self> for Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C, U> AsMut<Self> for Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// OptionEvent
//==============================================================================
pub trait OptionEvent {}
impl OptionEvent for crate::typelevel::NoneT {}
impl<E: AnyEvent> OptionEvent for E {}

//==============================================================================
// Event
//==============================================================================
/// Represents a configured and enabled EVSYS event which owns the involved
/// [`Channel`] and [`User`].
pub struct Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
    channel: C,
    user: U,
}

impl<C, U> Event<C, U>
where
    C: AnyChannel,
    U: AnyUser<ChId = C::Id>,
{
    /// Create a new [`Event`] from a [`Channel`] and [`User`]
    #[inline]
    pub fn new(channel: C, user: U) -> Self {
        Event { channel, user }
    }

    /// Release the owned [`Channel`] and [`User`], allowing them to be
    /// reused
    #[inline]
    pub fn free(self) -> (C, U) {
        (self.channel, self.user)
    }

    /// Checks the event channel for error flags.
    ///
    /// Note that asynchronous channels do not support error detection
    /// and will always return `Ok`.
    pub fn channel_error(&mut self) -> Result<(), Error> {
        self.channel.error()
    }

    /// Clears any pending errors for the event channel.
    pub fn clear_channel_errors(&mut self) {
        self.channel.clear_errors();
    }
}
