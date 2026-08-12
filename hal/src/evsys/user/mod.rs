use super::{
    Error,
    channel::{AnyChannel, Asynchronous, ChId, OptionChId, Resynchronized, Synchronous},
    evsys_controller::EvsysController,
};
use crate::typelevel::{Is, NoneT, Sealed};
use core::marker::PhantomData;
use paste::paste;

mod reg;
use reg::RegisterBlock;

//==============================================================================
// UsrId
//==============================================================================
pub trait UsrId: Sealed + Sized {
    const U8: u8;
    const USIZE: usize;
}

pub trait AsyncUsrId: UsrId {}
pub trait SyncUsrId: UsrId {}

//==============================================================================
// User
//==============================================================================
/// Trait representing a peripheral which can be configured to receive EVSYS
/// events.
///
/// # Safety
///
/// This trait must only be implemented on peripherals with the proper
/// [`UsrId`](s).
pub unsafe trait User<Id: UsrId + private::UserRegAccess>: Sealed {
    /// Configure the User to act on events from the given channel.
    ///
    /// Returns a [`UserMux`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    ///
    /// # Safety
    ///
    /// This method does not verify that the channel and user are compatible.
    /// [`AsyncUser::with_asynchronous_channel`],
    /// [`SyncUser::with_synchronous_channel`]
    /// or [`SyncUser::with_resynchronized_channel`] are the perfered safe API
    /// methods.
    unsafe fn with_channel_unchecked<C: AnyChannel>(
        controller: &mut EvsysController,
        _chan: C,
    ) -> Result<UserMux<Id, C::Id>, Error> {
        let mux = Id::take_register(&mut controller.user_regs)?;

        Ok(mux.to_channel())
    }
}

/// Trait for peripherals which accept asynchronous events
pub trait AsyncUser<Id: AsyncUsrId + private::UserRegAccess>: User<Id> {
    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports asynchronous events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`UserMux`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    fn with_asynchronous_channel<C: AnyChannel<Status = Asynchronous>>(
        controller: &mut EvsysController,
        chan: C,
    ) -> Result<UserMux<Id, C::Id>, Error> {
        // Always safe as method signature and trait bounds serves as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for asynchronous operation
        unsafe { <Self as User<Id>>::with_channel_unchecked(controller, chan) }
    }
}

/// Marker trait for peripherals which accept synchronous and resynchronized
/// events
pub trait SyncUser<Id: SyncUsrId + private::UserRegAccess>: User<Id> {
    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports synchronous events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`UserMux`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    fn with_synchronous_channel<C: AnyChannel<Status = Synchronous>>(
        controller: &mut EvsysController,
        chan: C,
    ) -> Result<UserMux<Id, C::Id>, Error> {
        // Always safe as method signature and trait bounds serves as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for synchronous operation
        unsafe { <Self as User<Id>>::with_channel_unchecked(controller, chan) }
    }

    /// Configure the User to act on events from the given channel, ensuring
    /// that the user supports resynchronized events and the channel has been
    /// configured as such.
    ///
    /// Returns a [`UserMux`] as validation that the EVSYS user multiplexer has
    /// been set to the provided channel.
    fn with_resynchronized_channel<C: AnyChannel<Status = Resynchronized>>(
        controller: &mut EvsysController,
        chan: C,
    ) -> Result<UserMux<Id, C::Id>, Error> {
        // Always safe as method signature and trait bounds serves as a
        // compile-time gaurentee that the user and channel both support
        // and are configured for resynchronized operation
        unsafe { <Self as User<Id>>::with_channel_unchecked(controller, chan) }
    }
}

//==============================================================================
// AnyUserMux
//==============================================================================
pub trait AnyUserMux: Sealed + Is<Type = SpecificUserMux<Self>> {
    type UsrId: UsrId;
    type ChId: ChId;
}

pub type SpecificUserMux<U> = UserMux<<U as AnyUserMux>::UsrId, <U as AnyUserMux>::ChId>;

pub type UserMuxId<U> = <U as AnyUserMux>::UsrId;
pub type UserMuxChId<U> = <U as AnyUserMux>::ChId;

impl<Id, C> Sealed for UserMux<Id, C>
where
    Id: UsrId,
    C: ChId,
{
}

impl<Id, C> AnyUserMux for UserMux<Id, C>
where
    Id: UsrId,
    C: ChId,
{
    type UsrId = Id;
    type ChId = C;
}

impl<Id, C> AsRef<Self> for UserMux<Id, C>
where
    Id: UsrId,
    C: ChId,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, C> AsMut<Self> for UserMux<Id, C>
where
    Id: UsrId,
    C: ChId,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// UserMux
//==============================================================================
pub struct UserMux<Id: UsrId, C: OptionChId> {
    regs: RegisterBlock<Id>,
    _channel: PhantomData<C>,
}

impl<Id: UsrId, C: OptionChId> UserMux<Id, C> {
    /// Updates the selected channel for this user mux
    pub fn to_channel<Other: ChId>(mut self) -> UserMux<Id, Other> {
        // Safe, as value is from ChId which is only implemented for valid
        // channels
        self.regs
            .user
            .write(|w| unsafe { w.channel().bits(Other::U8) });

        UserMux {
            regs: self.regs,
            _channel: PhantomData,
        }
    }
}

impl<Id: UsrId, C: ChId> UserMux<Id, C> {
    /// Disables the user multiplexer by resetting the register value
    pub fn disable(mut self) -> UserMux<Id, NoneT> {
        self.regs.user.write(|w| unsafe { w.channel().bits(0) });

        UserMux {
            regs: self.regs,
            _channel: PhantomData,
        }
    }
}

macro_rules! create_user_regs {
    (@new $name:ident, $id:literal) => {
        paste! {
            pub enum [< $name:camel >] {}

            impl Sealed for [< $name:camel >] {}
            impl UsrId for [< $name:camel >] {
                const U8: u8 = $id;
                const USIZE: usize = $id;

            }
            impl private::UserRegAccess for [< $name:camel >] {
                fn take_register(user_regs: &mut UserRegisters) -> Result<UserMux<Self, NoneT>, Error> {
                    Ok(UserMux {
                        regs: user_regs.[< $name:lower >].take().ok_or(Error::UserInUse)?,
                        _channel: PhantomData,
                    })
                }
            }
        }
    };

    // Internal rule for a asynchronous user
    (@async $name:ident) => {
        paste! {
            impl AsyncUsrId for [< $name:camel >] {}
        }
    };

    // Internal rule for synchronous user
    (@sync $name:ident) => {
        paste! {
            impl SyncUsrId for [< $name:camel >] {}
        }
    };

    // Async-only user
    (@user $name:ident, $id:literal, "A") => {
        paste! {
            create_user_regs!(@new $name, $id);
            create_user_regs!(@async $name);
        }
    };

    // Sync-only user
    (@user $name:ident, $id:literal, "S") => {
        paste! {
            create_user_regs!(@new $name, $id);
            create_user_regs!(@sync $name);
        }
    };

    // Sync/Async user
    (@user $name:ident, $id:literal, "AS") => {
        paste! {
            create_user_regs!(@new $name, $id);
            create_user_regs!(@async $name);
            create_user_regs!(@sync $name);
        }
    };

    // Rule to handle list processing
    (
        $(
            $name:ident, $id:literal, $type:literal
        );+
        $(,)?
    ) => {
        paste! {
            $(
                create_user_regs!(@user $name, $id, $type);
            )+

            pub struct UserRegisters {
                $( [< $name:lower >]: Option<RegisterBlock< [< $name:camel >] >>, )+
            }

            impl UserRegisters {
                pub(super) unsafe fn new() -> Self {
                    UserRegisters {
                        $( [< $name:lower >]: Some(RegisterBlock::new()), )+
                    }
                }
            }
        }
    };
}

//TODO: this is specifically for SAMD5x/E5x, it's probably different for
// SAMD21, etc and should be handled/defined differently. Also, not all of these
// need to be defined if the peripherals/channels are not useable (DMAC
// channels, TCC peripherals, etc)
create_user_regs! {
    RTC_TAMPER, 0, "A";
    PORT_EV0, 1, "A";
    PORT_EV1, 2, "A";
    PORT_EV2, 3, "A";
    PORT_EV3, 4, "A";
    DMAC_CH0, 5, "S";
    DMAC_CH1, 6, "S";
    DMAC_CH2, 7, "S";
    DMAC_CH3, 8, "S";
    DMAC_CH4, 9, "S";
    DMAC_CH5, 10, "S";
    DMAC_CH6, 11, "S";
    DMAC_CH7, 12, "S";
    // 13 - Reserved;
    CM4_TRACE_START, 14, "S";
    CM4_TRACE_STOP, 15, "S";
    CM4_TRACE_TRIG, 16, "S";
    TCC0_EV0, 17, "AS";
    TCC0_EV1, 18, "AS";
    TCC0_MC0, 19, "AS";
    TCC0_MC1, 20, "AS";
    TCC0_MC2, 21, "AS";
    TCC0_MC3, 22, "AS";
    TCC0_MC4, 23, "AS";
    TCC0_MC5, 24, "AS";
    TCC1_EV0, 25, "AS";
    TCC1_EV1, 26, "AS";
    TCC1_MC0, 27, "AS";
    TCC1_MC1, 28, "AS";
    TCC1_MC2, 29, "AS";
    TCC1_MC3, 30, "AS";
    TCC2_EV0, 31, "AS";
    TCC2_EV1, 32, "AS";
    TCC2_MC0, 33, "AS";
    TCC2_MC1, 34, "AS";
    TCC2_MC2, 35, "AS";
    TCC3_EV0, 36, "AS";
    TCC3_EV1, 37, "AS";
    TCC3_MC0, 38, "AS";
    TCC3_MC1, 39, "AS";
    TCC4_EV0, 40, "AS";
    TCC4_EV1, 41, "AS";
    TCC4_MC0, 42, "AS";
    TCC4_MC1, 43, "AS";
    TC0_EVU, 44, "AS";
    TC1_EVU, 45, "AS";
    TC2_EVU, 46, "AS";
    TC3_EVU, 47, "AS";
    TC4_EVU, 48, "AS";
    TC5_EVU, 49, "AS";
    TC6_EVU, 50, "AS";
    TC7_EVU, 51, "AS";
    PDEC_EVU0, 52, "AS";
    PDEC_EVU1, 53, "AS";
    PDEC_EVU2, 54, "AS";
    ADC0_START, 55, "AS";
    ADC0_SYNC, 56, "AS";
    ADC1_START, 57, "AS";
    ADC1_SYNC, 58, "AS";
    AC_SOC0, 59, "A";
    AC_SOC1, 60, "A";
    DAC_START0, 61, "AS";
    DAC_START1, 62, "AS";
    CCL_LUTIN0, 63, "AS";
    CCL_LUTIN1, 64, "AS";
    CCL_LUTIN2, 65, "AS";
    CCL_LUTIN3, 66, "AS"
}

mod private {
    use super::*;

    pub trait UserRegAccess: UsrId {
        fn take_register(user_regs: &mut UserRegisters) -> Result<UserMux<Self, NoneT>, Error>;
    }
}
