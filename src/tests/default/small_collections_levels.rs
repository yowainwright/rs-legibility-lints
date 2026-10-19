#![allow(no_small_collection_conversion, reason = "verify an explicit error override")]

use std::collections::HashSet;

#[deny(no_small_collection_conversion)]
fn rejected(value: u8) -> bool {
    HashSet::from([1, 2]).contains(&value)
}

fn main() {}
