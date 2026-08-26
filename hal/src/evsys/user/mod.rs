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
// UserId
//==============================================================================
pub trait UserId: Sealed + Sized {
    const U8: u8;
    const USIZE: usize;

    fn take_user(users: &mut Users) -> Result<User<Self, NoneT>, Error>;
}

pub trait AsyncUserId: UserId {}
pub trait SyncUserId: UserId {}

//==============================================================================
// AnyUser
//==============================================================================
pub trait AnyUser: Sealed + Is<Type = SpecificUser<Self>> {
    type UserId: UserId;
    type ChId: ChId;
}

pub type SpecificUser<U> = User<<U as AnyUser>::UserId, <U as AnyUser>::ChId>;

pub type UserUid<U> = <U as AnyUser>::UserId;
pub type UserChId<U> = <U as AnyUser>::ChId;

impl<Id, C> Sealed for User<Id, C>
where
    Id: UserId,
    C: ChId,
{
}

impl<Id, C> AnyUser for User<Id, C>
where
    Id: UserId,
    C: ChId,
{
    type UserId = Id;
    type ChId = C;
}

impl<Id, C> AsRef<Self> for User<Id, C>
where
    Id: UserId,
    C: ChId,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, C> AsMut<Self> for User<Id, C>
where
    Id: UserId,
    C: ChId,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// User
//==============================================================================
pub struct User<Id: UserId, C: OptionChId = NoneT> {
    regs: RegisterBlock<Id>,
    _channel: PhantomData<C>,
}

impl<Id: UserId> User<Id> {
    pub(crate) unsafe fn new() -> User<Id, NoneT> {
        User {
            regs: RegisterBlock::new(),
            _channel: PhantomData,
        }
    }
}

impl<Id: UserId, C: OptionChId> User<Id, C> {
    /// Updates the selected channel for this user mux
    pub fn with_channel<Other: ChId>(mut self) -> User<Id, Other> {
        // Safe, as value is from ChId which is only implemented for valid
        // channels
        self.regs
            .user
            .write(|w| unsafe { w.channel().bits(Other::U8) });

        User {
            regs: self.regs,
            _channel: PhantomData,
        }
    }
}

impl<Id: UserId, C: ChId> User<Id, C> {
    /// Disables the user multiplexer by resetting the register value
    pub fn disable(mut self) -> User<Id, NoneT> {
        self.regs.user.write(|w| unsafe { w.channel().bits(0) });

        User {
            regs: self.regs,
            _channel: PhantomData,
        }
    }
}

macro_rules! create_users {
    (@new $name:ident, $id:literal) => {
        paste! {
            pub enum [< $name:camel >] {}

            impl Sealed for [< $name:camel >] {}
            impl UserId for [< $name:camel >] {
                const U8: u8 = $id;
                const USIZE: usize = $id;

                fn take_user(users: &mut Users) -> Result<User<Self, NoneT>, Error> {
                    Ok(users.[< $name:lower >].take().ok_or(Error::UserInUse)?)
                }
            }
        }
    };

    // Internal rule for a asynchronous user
    (@async $name:ident) => {
        paste! {
            impl AsyncUserId for [< $name:camel >] {}
        }
    };

    // Internal rule for synchronous user
    (@sync $name:ident) => {
        paste! {
            impl SyncUserId for [< $name:camel >] {}
        }
    };

    // Async-only user
    (@user $name:ident, $id:literal, "A") => {
        paste! {
            create_users!(@new $name, $id);
            create_users!(@async $name);
        }
    };

    // Sync-only user
    (@user $name:ident, $id:literal, "S") => {
        paste! {
            create_users!(@new $name, $id);
            create_users!(@sync $name);
        }
    };

    // Sync/Async user
    (@user $name:ident, $id:literal, "AS") => {
        paste! {
            create_users!(@new $name, $id);
            create_users!(@async $name);
            create_users!(@sync $name);
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
                create_users!(@user $name, $id, $type);
            )+

            pub struct Users {
                $( [< $name:lower >]: Option<User< [< $name:camel >], NoneT>>, )+
            }

            impl Users {
                pub(super) unsafe fn new() -> Self {
                    Users {
                        $( [< $name:lower >]: Some(unsafe { User::new() }), )+
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
create_users! {
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
