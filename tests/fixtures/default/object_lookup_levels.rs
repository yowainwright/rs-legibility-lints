#![allow(max_expression_operators, reason = "isolate membership diagnostics")]
#![allow(prefer_object_lookup, reason = "verify an explicit error override")]

#[deny(prefer_object_lookup)]
fn rejected(value: u8) -> bool {
    value == 1 || value == 3 || value == 5
}

fn main() {}
