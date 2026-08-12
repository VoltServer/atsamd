use crate::{
    pac::{Evsys},
};

use crate::clock::v2::{
    apb::{ApbClk, DynApbId, ApbId},
    pclk::{PclkId, DynPclkId},
};

use seq_macro::seq;
use crate::typelevel::Sealed;
use super::channel::{ChId, SynchronousCh, Channel, Uninitialized};

use super::with_num_evsys_channels;
use super::with_num_evsys_synchronous_channels;

use super::user::UserRegisters;

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
    pub(super) user_regs: UserRegisters,
    _apbclk: ApbClk<Self>,
}

impl crate::typelevel::Sealed for EvsysController {}
impl ApbId for EvsysController {
    const DYN: DynApbId = DynApbId::EvSys;
}

impl EvsysController {
    pub fn init(mut evsys: Evsys, clock: ApbClk<Self>) -> Self {
        Self {
            evsys,
            user_regs: unsafe { UserRegisters::new() },
            _apbclk: clock,
        }
    }

    pub fn round_robin_scheduling(&mut self, yes: bool) {
        todo!()
    }

    pub fn swreset(&mut self) {
        todo!()
    }

    pub fn free(self) -> Evsys {
        todo!()
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
