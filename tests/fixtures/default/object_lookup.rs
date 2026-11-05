#![allow(max_expression_operators, reason = "isolate membership checks")]

fn eligible(status: &str) -> bool {
    status == "ready" || status == "queued" || status == "retrying"
}

fn reversed(value: u8) -> bool {
    1 == value || value == 3 || value == 5
}

fn parenthesized(value: u8) -> bool {
    (value == 1 || value == 3) || (value == 5 || value == 7)
}

fn negative(value: i32) -> bool {
    value == -1 || -3 == value || value == 5
}

fn characters(value: char) -> bool {
    value == 'a' || value == 'c' || value == 'e'
}

fn main() {}
