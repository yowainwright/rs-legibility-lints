#[deny(no_repeated_comprehension_filter)]
fn rejected(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn default_level(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

#[warn(no_repeated_comprehension_filter)]
fn locally_allowed(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    #[allow(no_repeated_comprehension_filter)]
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn main() {}
