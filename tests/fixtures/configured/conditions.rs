#![allow(max_expression_operators, reason = "exercise LEG002 independently")]

fn identity(value: bool) -> bool {
    value
}

fn conditions(left: bool, right: bool, other: bool, value: i32) {
    if left && right {}
    if left || right {}
    if !left && value > 0 {}
    if left && right && other {}
    if left && (right || other) {}
    if identity(left && right) && other {}
    if !identity(left && right && other) {}
    if !identity(left && right) {}
}

fn patterns(value: Option<i32>, left: bool, right: bool) {
    if let Some(value) = value
        && value > 0
        && left
        && right
    {}
}

fn main() {
    let left = true;
    let right = false;
    conditions(left, right, left, 1);
    patterns(Some(1), left, right);
}
