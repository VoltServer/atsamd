use atsamd_hal_macros::hal_cfg;

use super::super::evsys_controller::ChId;
use core::marker::PhantomData;
use paste::paste;

use crate::pac::{
    self, Evsys, Peripherals,
    evsys::{Busych, Intstatus, Swevt},
    evsys::{
        busych::BusychSpec, intstatus::IntstatusSpec, swevt::SwevtSpec,
    },
};

#[hal_cfg("evsys-d5x")]
use pac::evsys::channels as channel_regs;

use channel_regs::{
    Channel, channel::ChannelSpec,
    Chintenclr, chintenclr::ChintenclrSpec,
    Chintenset, chintenset::ChintensetSpec,
    Chintflag, chintflag::ChintflagSpec,
    Chstatus, chstatus::ChstatusSpec,
};

pub(super) trait Register<Id: ChId> {
    /// Get a shared reference to the underlying PAC object {
    fn evsys(&self) -> &Evsys;

    fn with_chid<F: FnOnce(&pac::evsys::Channels) -> R, R>(&mut self, fun: F) -> R {
        let ch = &self.evsys().channels(Id::USIZE);
        fun(ch)
    }
}

macro_rules! reg_proxy {
    (@new $reg:ident) => {
        paste! {
            /// Register proxy tied to a specific channel
            pub(super) struct [< $reg:camel Proxy >]<Id: ChId, REG> {
                #[allow(unused)]
                evsys: Evsys,
                _id: PhantomData<Id>,
                _reg: PhantomData<REG>,
            }

            impl<Id: ChId> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                /// Create a new register proxy
                #[inline]
                pub fn new() -> Self {
                    Self {
                        // SAFETY: This is safe as long as the register
                        // only reads/writes registers through
                        // the `with_chid` method.
                        evsys: unsafe { Peripherals::steal().evsys },
                        _id: PhantomData,
                        _reg: PhantomData,
                    }
                }
            }

            impl<Id: ChId> Register<Id> for [< $reg:camel Proxy >]<Id, [< $reg:camel >]> {
                fn evsys(&self) -> &Evsys {
                    &self.evsys
                }
            }
        }
    };

    // Internal rule for a Read-enabled register
    (@read_reg $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec >]: pac::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read(&mut self) -> channel_regs::[< $reg:lower >]::R {
                    self.with_chid(|d| d.[< $reg:lower >]().read())
                }
            }
        }
    };

    // Internal rule for a Write-enabled register
    (@write_reg $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec >]: pac::generic::Writable {
                #[inline]
                #[allow(dead_code)]
                pub fn write<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(&'w mut channel_regs::[< $reg:lower >]::W) -> &'w mut channel_regs::[< $reg:lower >]::W,
                {
                    self.with_chid(|d| d.[< $reg:lower >]().write(|w| func(w)));
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
                Id: ChId,
                [< $reg:camel Spec >]: pac::generic::Writable + pac::generic::Readable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn modify<F>(&mut self, func: F)
                where
                    for<'w> F: FnOnce(
                        &channel_regs::[< $reg:lower >]::R,
                        &'w mut channel_regs::[< $reg:lower >]::W,
                    ) -> &'w mut channel_regs::[< $reg:lower >]::W,
                {
                    self.with_chid(|d| d.[< $reg:lower >]().modify(|r, w| func(r, w)));
                }
            }
        }
    };

    // Internal rule for read-enabled bit
    (@read_bit $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where Id: ChId, [< $reg:camel Spec >]: pac::generic::Readable {
                #[inline]
                #[allow(dead_code)]
                pub fn read_bit(&self) -> bool {
                    self.evsys.[< $reg:lower >]().read().bits() & (1 << Id::U8) != 0
                }
            }
        }
    };

    // Internal rule for write-enabled bit
    (@write_bit $reg:ident) => {
        paste! {
            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where
                Id: ChId,
                [< $reg:camel Spec >]: pac::generic::Writable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn set_bit(&mut self) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    unsafe {
                        self.evsys.[< $reg:lower >]().write(|w| w.bits(1 << Id::U8));
                    }
                }
            }
        }
    };

    // Read-only bit
    ($reg:ident, bit, r) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_bit $reg);
        }
    };

    // Write-only bit
    ($reg:ident, bit, w) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@write_bit $reg);
        }
    };

    // Read-write bit
    ($reg:ident, bit, rw) => {
        paste! {
            reg_proxy!(@new $reg);
            reg_proxy!(@read_bit $reg);

            impl<Id> [< $reg:camel Proxy >]<Id, [< $reg:camel >]> where
                Id: ChId,
                [< $reg:camel Spec >]: pac::generic::Readable + pac::generic::Writable
            {
                #[inline]
                #[allow(dead_code)]
                pub fn set_bit(&mut self) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    unsafe {
                        self.evsys.[< $reg:lower >]().modify(|r, w| w.bits(r.bits() | (1 << Id::U8)));
                    }
                }

                #[inline]
                #[allow(dead_code)]
                pub fn clear_bit(&mut self) {
                    // SAFETY: This is safe because we are only writing
                    // to the bit controlled by the channel.
                    unsafe {
                        self.evsys.[< $reg:lower >]().modify(|r, w| w.bits(r.bits() & !(1 << Id::U8)));
                    }
                }
            }
        }
    };
}

reg_proxy!(channel, register, rw);
reg_proxy!(chintenclr, register, rw);
reg_proxy!(chintenset, register, rw);
reg_proxy!(chintflag, register, rw);
reg_proxy!(chstatus, register, r);

reg_proxy!(intstatus, bit, r);
reg_proxy!(busych, bit, r);
reg_proxy!(swevt, bit, w);

pub(super) struct RegisterBlock<Id: ChId> {
    pub channel: ChannelProxy<Id, Channel>,
    pub chintenclr: ChintenclrProxy<Id, Chintenclr>,
    pub chintenset: ChintensetProxy<Id, Chintenset>,
    pub chintflag: ChintflagProxy<Id, Chintflag>,
    pub chstatus: ChstatusProxy<Id, Chstatus>,
    pub intstatus: IntstatusProxy<Id, Intstatus>,
    pub busych: BusychProxy<Id, Busych>,
    pub swevt: SwevtProxy<Id, Swevt>,
}

impl<Id: ChId> RegisterBlock<Id> {
    pub(super) fn new(_id: PhantomData<Id>) -> Self {
        Self {
            channel: ChannelProxy::new(),
            chintenclr: ChintenclrProxy::new(),
            chintenset: ChintensetProxy::new(),
            chintflag: ChintflagProxy::new(),
            chstatus: ChstatusProxy::new(),
            intstatus: IntstatusProxy::new(),
            busych: BusychProxy::new(),
            swevt: SwevtProxy::new(),
        }
    }
}

impl<Id: ChId> Drop for RegisterBlock<Id> {
    fn drop(&mut self) {
        // Disable event generation for the channel
        self.channel.modify(|_, w| w.evgen().none());
    }
}
