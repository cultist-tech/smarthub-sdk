pub use core_impl::MultiFungibleToken;
pub use macros::*;

pub use self::core::MultiFungibleTokenCore;
pub use self::events::*;
pub use self::metadata::*;
pub use self::receiver::MultiFungibleTokenReceiver;
pub use self::resolver::MultiFungibleTokenResolver;

pub mod core;
pub mod core_impl;
pub mod events;
pub mod macros;
pub mod metadata;
pub mod receiver;
pub mod resolver;