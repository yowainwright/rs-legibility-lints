fn at_limit(values: [bool; 5]) -> bool {
    values[0] && values[1] && values[2] && values[3] && values[4]
}

fn above_limit(values: [bool; 6]) -> bool {
    values[0] && values[1] && values[2] && values[3] && values[4] && values[5]
}

fn comparisons(values: [i32; 3]) -> bool {
    values[0] > 0 && values[1] > 0 && values[2] > 0
}

fn negations(values: [bool; 3]) -> bool {
    !values[0] && !values[1] && !values[2]
}

fn arithmetic(value: i32) -> i32 {
    value + 1 + 2 + 3 + 4 + 5 + 6
}

fn bitwise(value: u32) -> u32 {
    !!!!!!value | value & value ^ value << 1 >> 1
}

fn both(left: bool, right: bool) -> bool {
    left && right
}

fn separate_arguments(values: [bool; 4]) -> bool {
    both(
        values[0] && values[1] && values[2] && values[3],
        values[0] || values[1] || values[2] || values[3],
    )
}

fn closure(values: [bool; 3]) -> bool {
    let check = || !values[0] && !values[1] && !values[2];
    check()
}

fn separate_blocks(values: [bool; 3]) -> bool {
    let left = { !values[0] && !values[1] };
    let right = { values[0] || values[1] || values[2] };
    left && right
}

fn parenthesized(values: [bool; 6]) -> bool {
    (values[0] && values[1] && values[2] && values[3]) && (values[4] && values[5])
}

fn conditional(values: [bool; 6]) -> bool {
    if values[0] && values[1] && values[2] && values[3] && values[4] && values[5] {
        true
    } else {
        false
    }
}

async fn asynchronous(values: [bool; 3]) -> bool {
    !values[0] && !values[1] && !values[2]
}

#[allow(max_expression_operators, reason = "verify native lint suppression")]
fn suppressed(values: [bool; 6]) -> bool {
    values[0] && values[1] && values[2] && values[3] && values[4] && values[5]
}

macro_rules! generated {
    ($value:expr) => {
        $value && $value && $value && $value && $value && $value
    };
}

fn main() {
    let _ = at_limit([true; 5]);
    let _ = above_limit([true; 6]);
    let _ = comparisons([1; 3]);
    let _ = negations([true; 3]);
    let _ = arithmetic(1);
    let _ = bitwise(1);
    let _ = separate_arguments([true; 4]);
    let _ = closure([true; 3]);
    let _ = separate_blocks([true; 3]);
    let _ = parenthesized([true; 6]);
    let _ = conditional([true; 6]);
    let _ = asynchronous([true; 3]);
    let _ = suppressed([true; 6]);
    let _ = generated!(true);
}
