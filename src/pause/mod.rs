pub use pause_impl::*;
pub use macros::*;

pub mod pause_impl;
pub mod macros;

pub trait ContractPause {
    fn is_paused(&self) -> bool;

    fn set_is_paused(&mut self, pause: bool) -> bool;
}