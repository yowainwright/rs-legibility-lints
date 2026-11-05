#![allow(max_expression_operators, reason = "exercise LEG002 independently")]

fn conditions(left: bool, right: bool, value: i32) {
    if left && right {}
    if left || right {}
    if left && (right || left) {}
    if left {}
    if !left {}
    if value > 0 {}
    if left & right {}
    let can_proceed = left && right;
    if can_proceed {}
}

fn identity(value: bool) -> bool {
    value
}

fn take_closure(check: impl FnOnce() -> bool) -> bool {
    check()
}

fn calls(left: bool, right: bool) {
    if identity(left && right) {}
    if (left || right).then_some(left).is_some() {}
    if [left && right][0] {}
    if (left && right) == left {}
    if take_closure(|| left && right) {}
    if {
        let ready = left;
        ready && right
    } {}
}

fn nested(left: bool, right: bool, other: bool) {
    if left {
        if right && other {}
    } else if left || right {
    }
}

fn patterns(value: Option<i32>, left: bool, right: bool) {
    if let Some(value) = value {
        let _ = value;
    }
    if let Some(value) = value
        && value > 0
        && left
        && right
    {}
    if left
        && right
        && let Some(value) = value
        && value > 0
    {}
    if let Some(flag) = Some(left && right) {
        let _ = flag;
    }
}

fn other_contexts(left: bool, right: bool) {
    while left && right {
        break;
    }
    match left {
        true if left && right => (),
        _ => (),
    }
}

#[allow(hoist_if_operators, reason = "verify native lint suppression")]
fn suppressed(left: bool, right: bool) {
    if left && right {}
}

macro_rules! generated_condition {
    ($left:expr, $right:expr) => {
        $left && $right
    };
}

macro_rules! generated_if {
    ($left:expr, $right:expr) => {
        if $left && $right {}
    };
}

fn main() {
    let left = true;
    let right = false;
    conditions(left, right, 1);
    calls(left, right);
    nested(left, right, left);
    patterns(Some(1), left, right);
    other_contexts(left, right);
    suppressed(left, right);
    if generated_condition!(true, false) {}
    generated_if!(true, false);
}
