// use near_sdk::env;
use std::fmt::{ Formatter, Error, Display };

#[derive(Debug)]
pub enum MtError {
    NotFoundToken,
}

impl Display for MtError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            MtError::NotFoundToken => f.write_str("Not found token"),
        }
    }
}

// impl MtError {
//   pub fn panic(&self) {
//     let error = self;
//
//     env::panic_str(&error.to_string());
//   }
// }