use crate::{
    pac::{Evsys},
};

use crate::clock::v2::{
    apb::{ApbClk, DynApbId, ApbId},
    pclk::{PclkId, DynPclkId},
};

use seq_macro::seq;
use crate::typelevel::Sealed;
use super::channel::{Channel, Uninitialized};

use super::with_num_evsys_channels;
use super::with_num_evsys_synchronous_channels;

pub trait ChId: Sealed {
    const U8: u8;
    const USIZE: usize;
}

/// Marker trait for channels that support the synchronus and resynchronized paths
pub trait SynchronusCh: ChId + PclkId {}

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

                impl SynchronusCh for Ch~N {}
            )*
        });
    };
}
with_num_evsys_synchronous_channels!(mark_synchronous_channels);




pub struct EvsysController {
    evsys: Evsys,
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
            _apbclk: clock,
        }
    }

    pub fn free(self) -> Evsys {
        todo!()
    }
}

