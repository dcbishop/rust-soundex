#![crate_id = "soundex#0.0.1"]
#![crate_type = "lib"]
#![desc = "Rust soundex implementation."]
#![license = "CC0"]
#![feature(globs)]
#![feature(macro_rules)]

extern crate collections;

pub use soundex::*;

mod soundex;
