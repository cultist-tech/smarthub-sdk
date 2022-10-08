pub use macros::*;

pub use self::base::{ RentFeatureCore, RentFeatureResolve, RentFeature };
pub use self::enumeration::RentFeatureEnumeration;
pub use self::meta::*;

pub mod enumeration;
pub mod base;
pub mod meta;

pub mod utils;
pub use self::utils::*;

mod macros;

pub mod events;
pub use self::events::*;

mod test;

pub trait RentFeatureReceiver {
    fn rent_on_lock(&self, token_id: TokenId, locked: bool);
}
