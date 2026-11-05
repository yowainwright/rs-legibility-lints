#![allow(max_expression_operators, reason = "isolate membership exclusions")]

fn below_limit(status: &str) -> bool {
    status == "ready" || status == "queued"
}

fn different_subject(value: u8, other: u8) -> bool {
    value == 1 || value == 3 || other == 5
}

fn invalid_suffix(value: u8, other: u8) -> bool {
    value == 1 || value == 3 || value == 5 || other == 7
}

fn inequalities(value: u8) -> bool {
    value == 1 || value == 3 || value != 5
}

fn constants(value: u8) -> bool {
    const FIRST: u8 = 1;
    value == FIRST || value == 3 || value == 5
}

fn fields(value: (u8,)) -> bool {
    value.0 == 1 || value.0 == 3 || value.0 == 5
}

fn calls(next: impl Fn() -> u8) -> bool {
    next() == 1 || next() == 3 || next() == 5
}

struct Custom(u8);

impl PartialEq<u8> for Custom {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

fn overloaded(value: Custom) -> bool {
    value == 1 || value == 3 || value == 5
}

fn owned_strings(value: String) -> bool {
    value == "ready" || value == "queued" || value == "retrying"
}

fn floats(value: f64) -> bool {
    value == 1.0 || value == 3.0 || value == 5.0
}

fn bitwise(value: u8) -> bool {
    (value == 1) | (value == 3) | (value == 5)
}

fn mixed_conditions(value: u8, enabled: bool) -> bool {
    value == 1 || value == 3 && enabled || value == 5
}

macro_rules! generated {
    ($value:expr) => {
        $value == 1 || $value == 3 || $value == 5
    };
}

macro_rules! literal {
    () => { 5 };
}

fn macro_literal(value: u8) -> bool {
    value == 1 || value == 3 || value == literal!()
}

#[allow(prefer_object_lookup, reason = "verify native lint suppression")]
fn suppressed(value: u8) -> bool {
    value == 1 || value == 3 || value == 5
}

fn accepted(value: u8) -> bool {
    matches!(value, 1 | 3 | 5)
}

fn main() {
    let _ = generated!(0);
}
