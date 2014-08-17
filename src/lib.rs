#![crate_name = "soundex"]
#![crate_type = "lib"]
#![desc = "Rust soundex implementation."]
#![license = "CC0"]
#![feature(globs)]
#![feature(macro_rules)]

extern crate collections;

pub use soundex::*;

mod soundex;
