fn at_limit(left: bool, right: bool) -> bool {
    left && right
}

fn above_limit(values: [bool; 3]) -> bool {
    values[0] && values[1] && values[2]
}

fn borrowed(value: &bool) -> bool {
    !*value
}

fn check(value: bool) -> bool {
    value
}

fn call_at_limit(left: bool, right: bool) -> bool {
    check(left && right)
}

fn call_above_limit(values: [bool; 3]) -> bool {
    values[0] && check(values[1] && values[2])
}

fn main() {
    let left = true;
    let right = false;
    let _ = at_limit(left, right);
    let _ = above_limit([true; 3]);
    let _ = borrowed(&true);
    let _ = call_at_limit(left, right);
    let _ = call_above_limit([true; 3]);
}
