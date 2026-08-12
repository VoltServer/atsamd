use super::{channel::{AnyChannel, ChannelId}, user::{AnyUserMux, UserMuxId}};
use crate::typelevel::{Sealed, Is};


//==============================================================================
// AnyEvent
//==============================================================================
/// Type-level abstraction over [`Event`]
pub trait AnyEvent: Sealed + Is<Type = SpecificEvent<Self>> {
    type Channel: AnyChannel;
    type UserMux: AnyUserMux<ChId = EventChannelId<Self>>;
}

pub type SpecificEvent<E> = Event<<E as AnyEvent>::Channel, <E as AnyEvent>::UserMux>;

pub type EventChannel<E> = <E as AnyEvent>::Channel;
pub type EventChannelId<E> = ChannelId<EventChannel<E>>;
pub type EventUserMux<E> = <E as AnyEvent>::UserMux;
pub type EventUserId<E> = UserMuxId<EventUserMux<E>>;

impl<C, U> Sealed for Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
}

impl<C, U> AnyEvent for Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
    type Channel = C;
    type UserMux = U;
}

impl<C, U> AsRef<Self> for Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C, U> AsMut<Self> for Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Event
//==============================================================================
/// Represents a configured and enabled EVSYS event which owns the involved
/// [`Channel`] and [`UserMux`].
pub struct Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
    channel: C,
    user_mux: U,
}

impl<C, U> Event<C, U>
where
    C: AnyChannel,
    U: AnyUserMux<ChId = C::Id>,
{
    /// Create a new [`Event`] from a [`Channel`] and [`UserMux`]
    #[inline]
    pub fn new(channel: C, user_mux: U) -> Self {
        Event {
            channel,
            user_mux,
        }
    }

    /// Release the owned [`Channel`] and [`UserMux`], allowing them to be reused
    #[inline]
    pub fn free(self) -> (C, U) {
        (self.channel, self.user_mux)
    }
}
