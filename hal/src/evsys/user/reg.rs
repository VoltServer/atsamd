use super::UserId;
use core::marker::PhantomData;
use paste::paste;

use crate::pac::{
    self, Evsys, Peripherals,
    evsys::{User, user::UserSpec},
};

use pac::evsys as user_regs;

pub(super) trait Register<Id: UserId> {
    /// Get a shared reference to the underlying PAC object
    fn evsys(&self) -> &Evsys;

    fn with_usrid<F: FnOnce(&pac::evsys::User) -> R, R>(&mut self, fun: F) -> R {
        let usr = &self.evsys().user(Id::USIZE);
        fun(usr)
    }
}

macro_rules! reg_proxy {
    (@new $reg:ident) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $reg:camel Proxy >]<Id: UserId, REG> {
                #[allow(unused)]
                evsys: Evsys,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: UserId> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                /// Create a new register proxy
                #[inline]
                pub fn new() -> Self {
                    Self {
                        // SAFETY: This is safe as long as the register
                        // only reads/writes registers through
                        // the `with_usrid` method.
                        evsys: unsafe { Peripherals::steal().evsys },
                        _id: PhantomData,
                        _reg: PhantomData,
                    }
                }
            }

            impl<Id: UserId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                fn evsys(&self) -> &Evsys {
                    &self.evsys
                }
            }
        }
    };

    // Internal rule for a Read-enabled register
    (@read_reg $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: UserId, [< $reg:camel Spec >]: pac::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read(&mut self) -> user_regs::[< $reg:lower >]::R {
                    self.with_usrid(|d| d.read())
                }
            }
        }
    };

    // Internal rule for a Write-enabled register
    (@write_reg $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: UserId, [< $reg:camel Spec >]: pac::generic::Writable {
                #[inline]
                #[allow(dead_code)]
                pub fn write<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(&'w mut user_regs::[< $reg:lower >]::W) -> &'w mut user_regs::[< $reg:lower >]::W,
                {
                    self.with_usrid(|d| d.write(|w| func(w)));
                }
            }
        }
    };

    // Read-only register
    ($reg:ident, register, r) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_reg $reg);
        }
    };

    // Write-only register
    ($reg:ident, register, w) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@write_reg $reg);
        }
    };

    // Read-write register
    ($reg:ident, register, rw) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_reg $reg);
            reg_proxy!(@write_reg $reg);

            impl<Id>[< $reg:camel Proxy >]<Id, [< $reg:camel >]> where
                Id: UserId,
                [< $reg:camel Spec >]: pac::generic::Writable + pac::generic::Readable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn modify<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(
                        &user_regs::[< $reg:lower >]::R,
                        &'w mut user_regs::[< $reg:lower >]::W,
                    ) -> &'w mut user_regs::[< $reg:lower >]::W,
                {
                    self.with_usrid(|d| d.modify(|r, w| func(r, w)));
                }
            }
        }
    };
}

reg_proxy!(user, register, rw);

pub(super) struct RegisterBlock<Id: UserId> {
    pub user: UserProxy<Id, User>,
}

impl<Id: UserId> RegisterBlock<Id> {
    pub(super) fn new() -> Self {
        Self {
            user: UserProxy::new(),
        }
    }
}

impl<Id: UserId> Drop for RegisterBlock<Id> {
    fn drop(&mut self) {
        // Disable event propagation for this user when dropped
        // SAFTEY: Safe since this is the value at reset
        self.user.write(|w| unsafe { w.channel().bits(0) });
    }
}
