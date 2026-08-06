use atsamd_hal_macros::hal_cfg;

mod channel;
mod evsys_controller;
mod event;

pub use channel::*;
pub use evsys_controller::*;
pub use event::*;

#[hal_cfg("dmac-d5x")]
#[macro_export]
macro_rules! with_num_evsys_channels {
    ($some_macro:ident) => {
        $some_macro! {32}
    }
}

#[hal_cfg("dmac-d5x")]
#[macro_export]
macro_rules! with_num_evsys_synchronus_channels {
    ($some_macro:ident) => {
        $some_macro! {12}
    }
}

