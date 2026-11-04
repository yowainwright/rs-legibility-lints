#![allow(no_computed_values, reason = "verify explicit error overrides")]

#[deny(no_computed_values)]
fn rejected(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    subtotal + tax + shipping
}

fn main() {}
