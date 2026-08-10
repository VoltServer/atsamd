use crate::{
    pac::{Evsys},
};

use crate::clock::v2::{
    apb::{ApbClk, DynApbId, ApbId},
    pclk::{PclkId, DynPclkId},
};

pub trait ChId: PclkId {
    const U8: u8;
    const USIZE: usize;
}

/// Marker trait for channels that support the synchronus and resynchronized paths
pub trait SynchronusCh: ChId {}

macro_rules! define_channel_struct {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                pub enum Ch~N {}

                impl ChId for CH~N {
                    const U8: u8 = N;
                    const USIZE: usize = N;
                }

                impl PclkId for CH~N {
                    const DYN: DynPclkId = DynPclkId::EvSys~N;
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

macro_rules! mark_synchronus_channels {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                impl SynchronusCh for Ch~N {}
            )*
        });
    };
}




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

