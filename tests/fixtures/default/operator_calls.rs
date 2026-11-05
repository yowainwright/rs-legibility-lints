#![warn(max_expression_operators)]

fn function_calls(v: [bool; 6]) -> bool {
    keep(keep(v[0] && v[1] && v[2] && v[3] && v[4] && v[5]))
}

fn method_calls(v: [bool; 6]) -> bool {
    (v[0] && v[1] && v[2] && v[3] && v[4] && v[5]).then_some(1).is_some()
}

fn keep(value: bool) -> bool { value }

fn main() {}
