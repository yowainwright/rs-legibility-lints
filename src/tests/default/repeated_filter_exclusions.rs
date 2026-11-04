#![warn(no_repeated_comprehension_filter)]

fn different_collection(values: &[i32], other: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = other.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn different_predicate(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value >= 0).collect();
    let third: Vec<_> = values.iter().filter(|value| **value >= 1).collect();
    let fourth: Vec<_> = values.iter().filter(|value| 1 <= **value).collect();
    let _ = (first, second, third, fourth);
}

fn mutable_binding(mut values: Vec<i32>) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
    values.push(0);
}

fn mutable_reference(values: &mut [i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn intervening_statement(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    std::hint::black_box(values);
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn captured_threshold(values: &[i32], threshold: i32) {
    let first: Vec<_> = values.iter().filter(|value| **value > threshold).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > threshold).collect();
    let _ = (first, second);
}

fn effectful_predicate(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| std::hint::black_box(**value) > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| std::hint::black_box(**value) > 0).collect();
    let _ = (first, second);
}

#[allow(max_array_chain_depth)]
fn extra_adapter(values: &[i32]) {
    let first: Vec<_> = values.iter().copied().filter(|value| *value > 0).collect();
    let second: Vec<_> = values.iter().copied().filter(|value| *value > 0).collect();
    let _ = (first, second);
}

fn different_output(values: &[i32]) {
    let first: std::collections::BTreeSet<_> = values.iter().filter(|value| **value > 0).collect();
    let second: std::collections::BTreeSet<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

#[derive(PartialEq)]
struct Custom(i32);

impl PartialEq<i32> for Custom {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

fn custom_comparison(values: &[Custom]) {
    let first: Vec<_> = values.iter().filter(|value| **value == 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value == 0).collect();
    let _ = (first, second);
}

struct CustomCollection(Vec<i32>);

impl CustomCollection {
    fn iter(&self) -> std::slice::Iter<'_, i32> {
        self.0.iter()
    }
}

fn custom_iterator(values: &CustomCollection) {
    let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

macro_rules! pass {
    ($value:expr) => { $value };
}

fn macro_collection(values: &[i32]) {
    let first: Vec<_> = pass!(values).iter().filter(|value| **value > 0).collect();
    let second: Vec<_> = pass!(values).iter().filter(|value| **value > 0).collect();
    let _ = (first, second);
}

fn macro_literal(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(|value| **value > pass!(0)).collect();
    let second: Vec<_> = values.iter().filter(|value| **value > pass!(0)).collect();
    let _ = (first, second);
}

fn macro_predicate(values: &[i32]) {
    let first: Vec<_> = values.iter().filter(pass!(|value: &&i32| **value > 0)).collect();
    let second: Vec<_> = values.iter().filter(pass!(|value: &&i32| **value > 0)).collect();
    let _ = (first, second);
}

fn main() {}
