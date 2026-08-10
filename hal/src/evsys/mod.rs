use atsamd_hal_macros::hal_cfg;

pub use channel::*;
pub use evsys_controller::*;
pub use event::*;

pub mod channel;
pub mod evsys_controller;
pub mod event;


#[hal_cfg("dmac-d5x")]
macro_rules! with_num_evsys_channels {
    ($some_macro:ident) => {
        $some_macro! {32}
    };
}
pub(super) use with_num_evsys_channels;

#[hal_cfg("dmac-d5x")]
macro_rules! with_num_evsys_synchronous_channels {
    ($some_macro:ident) => {
        $some_macro! {12}
    };
}
pub(super) use with_num_evsys_synchronous_channels;

