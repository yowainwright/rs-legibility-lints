#![warn(no_repeated_comprehension_filter)]

fn selected(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn array_input(values: [u8; 4]) {
    let first: Vec<_> = values.iter().filter(|value| 2 <= **value).collect();
    let second: Vec<_> = values.iter().filter(|item| 2 <= **item).collect();
    let _ = (first, second);
}

fn vector_input(values: Vec<i32>) {
    let first: Vec<_> = values.iter().filter(|value| **value != -1).collect();
    let second: Vec<_> = values.iter().filter(|item| **item != -1).collect();
    let _ = (first, second);
}

fn string_input(values: &[&str]) {
    let first: Vec<_> = values.iter().filter(|value| **value == "ready").collect();
    let second: Vec<_> = values.iter().filter(|item| **item == "ready").collect();
    let _ = (first, second);
}

fn main() {}
