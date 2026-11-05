fn check(value: bool) -> bool {
    value
}

fn at_limit(values: [bool; 5]) -> bool {
    values[0] && check(values[1] && values[2] && values[3] && values[4])
}

fn above_limit(values: [bool; 6]) -> bool {
    values[0] && check(values[1] && values[2] && values[3] && values[4] && values[5])
}

fn nested_calls(values: [bool; 6]) -> bool {
    values[0] && check(check(values[1] && values[2] && values[3] && values[4] && values[5]))
}

fn method_at_limit(values: [bool; 6]) -> Option<bool> {
    (values[0] && values[1] && values[2]).then_some(values[3] && values[4] && values[5])
}

fn method_above_limit(values: [bool; 7]) -> Option<bool> {
    (values[0] && values[1] && values[2] && values[3])
        .then_some(values[4] && values[5] && values[6])
}

fn take_closure(check: impl FnOnce() -> bool) -> bool {
    check()
}

fn closure_body_is_separate(values: [bool; 6]) -> bool {
    values[0] && take_closure(|| values[1] && values[2] && values[3] && values[4] && values[5])
}

fn main() {
    let _ = at_limit([true; 5]);
    let _ = above_limit([true; 6]);
    let _ = nested_calls([true; 6]);
    let _ = method_at_limit([true; 6]);
    let _ = method_above_limit([true; 7]);
    let _ = closure_body_is_separate([true; 6]);
}
