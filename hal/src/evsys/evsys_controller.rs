use crate::pac::Evsys;

use crate::clock::v2::{
    apb::{ApbClk, ApbId, DynApbId},
    pclk::{DynPclkId, PclkId},
};

use super::channel::{
    ChId, Channel, SynchronousCh, Asynchronous, Synchronous, Resynchronized, Uninitialized,
    AnyChannel
};
use crate::typelevel::Sealed;
use seq_macro::seq;

use super::with_num_evsys_channels;
use super::with_num_evsys_synchronous_channels;

use super::user::{UserId, AsyncUserId, SyncUserId, User, Users};

macro_rules! define_channel_struct {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                pub enum Ch~N {}

                impl Sealed for Ch~N {}

                impl ChId for Ch~N {
                    const U8: u8 = N;
                    const USIZE: usize = N;
                }

            )*


            pub struct Channels(
                #(
                    pub Channel<Ch~N, Uninitialized>,
                )*
            );
        });
    };
}
with_num_evsys_channels!(define_channel_struct);

macro_rules! mark_synchronous_channels {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                impl PclkId for Ch~N {
                    const DYN: DynPclkId = DynPclkId::EvSys~N;
                }

                impl SynchronousCh for Ch~N {}
            )*
        });
    };
}
with_num_evsys_synchronous_channels!(mark_synchronous_channels);

pub struct EvsysController {
    evsys: Evsys,
    //pub(super) user_regs: UserRegisters,
    users: Users,
    _apbclk: ApbClk<Self>,
}

impl crate::typelevel::Sealed for EvsysController {}
impl ApbId for EvsysController {
    const DYN: DynApbId = DynApbId::EvSys;
}

impl EvsysController {
    pub fn init(evsys: Evsys, clock: ApbClk<Self>) -> Self {
        Self {
            evsys,
            //user_regs: unsafe { UserRegisters::new() },
            users: unsafe { Users::new() },
            _apbclk: clock,
        }
    }

    pub fn round_robin_scheduling(&mut self, yes: bool) {
        self.evsys.prictrl().write(|w| w.rren().bit(yes));
    }

    pub fn swreset(&mut self) {
        self.evsys.ctrla().write(|w| w.swrst().set_bit());
        while self.evsys.ctrla().read().swrst().bit_is_set() {}
    }

    /// Obtain a User to act on events from the given channel.
    ///
    /// Returns a [`User`] with the specified channel Id as validation that the EVSYS
    /// user multiplexer has been set to the provided channel.
    ///
    /// # Safety
    ///
    /// This method does not verify that the channel and user are compatible.
    /// [`EvsysController::with_asynchronous_channel`],
    /// [`EvsysController::with_synchronous_channel`]
    /// or [`EvsysController::with_resynchronized_channel`] are the perfered safe API
    /// methods.
    pub unsafe fn with_channel_unchecked<C, Uid>(
        &mut self,
        _chan: &C,
    ) -> Result<User<Uid, C::Id>, super::Error>
    where
        C: AnyChannel,
        Uid: UserId,
    {
        let mux = Uid::take_user(&mut self.users)?;

        Ok(mux.with_channel())
    }

    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports asynchronous events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`User`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    pub fn with_asynchronous_channel<C, Uid>(
        &mut self,
        chan: &C,
    ) -> Result<User<Uid, C::Id>, super::Error>
    where
        C: AnyChannel<Status = Asynchronous>,
        Uid: AsyncUserId,
    {
        // Always safe as method signature and trait bounds serve as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for synchronous operation
        unsafe { self.with_channel_unchecked(chan) }
    }

    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports synchronous events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`User`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    pub fn with_synchronous_channel<C, Uid>(
        &mut self,
        chan: &C,
    ) -> Result<User<Uid, C::Id>, super::Error>
    where
        C: AnyChannel<Status = Synchronous>,
        Uid: SyncUserId,
    {
        // Always safe as method signature and trait bounds serve as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for synchronous operation
        unsafe { self.with_channel_unchecked(chan) }
    }

    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports resynchronized events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`User`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    pub fn with_resynchronized_channel<C, Uid>(
        &mut self,
        chan: &C,
    ) -> Result<User<Uid, C::Id>, super::Error>
    where
        C: AnyChannel<Status = Resynchronized>,
        Uid: SyncUserId,
    {
        // Always safe as method signature and trait bounds serve as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for resynchronized operation
        unsafe { self.with_channel_unchecked(chan) }
    }
}

macro_rules! define_split {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            /// Split the EVSYS into individual channels
            #[inline]
            pub fn split(&mut self) -> Channels {
                Channels(
                    #(
                        super::channel::new_chan(core::marker::PhantomData),
                    )*
                )
            }
        });
    };
}

impl EvsysController {
    with_num_evsys_channels!(define_split);
}
