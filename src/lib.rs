// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate polarization;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate num;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
#[macro_use]
extern crate assert_approx_eq;
#[macro_use]
extern crate prettytable;

pub mod errors;
pub mod from_toml;
pub mod validate;
pub mod report;
