use crate::{
    pac::{Evsys},
};

use crate::clock::v2::{
    apb::{ApbClk, DynApbId, ApbId},
};

pub trait ChId {
    const U8: u8;
    const USIZE: usize;
}

macro_rules! define_channel_struct {
    ($num_channels:literal) => {
        seq!(N in 0..$num_channels {
            #(
                pub enum Ch~N {}

                impl ChId for CH~N {
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

