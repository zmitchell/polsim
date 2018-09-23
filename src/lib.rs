// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate polarization;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate num;
pub mod errors;
pub mod from_toml;
pub mod validate;
