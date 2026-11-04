fn at_limit(values: &[i32]) -> Vec<i32> {
    values.iter().map(|value| *value + 1).collect()
}

fn above_limit(values: &[i32]) -> bool {
    values
        .iter()
        .filter(|value| **value > 0)
        .map(|value| *value * 2)
        .any(|value| value > 100)
}

fn longer_chain(values: Vec<i32>) -> usize {
    values
        .into_iter()
        .filter(|value| *value > 0)
        .map(|value| value * 2)
        .take(3)
        .count()
}

fn named_steps(values: &[i32]) -> bool {
    let positive = values.iter().filter(|value| **value > 0);
    let mut doubled = positive.map(|value| *value * 2);
    doubled.any(|value| value > 100)
}

fn nested_closure(values: &[i32]) -> usize {
    values
        .iter()
        .map(|value| (0..*value).filter(|n| *n > 0).map(|n| n + 1).count())
        .sum()
}

fn non_iterator_parent(values: &[i32]) -> i32 {
    values
        .iter()
        .copied()
        .filter(|value| *value > 0)
        .max()
        .unwrap_or(0)
}

fn generic(values: impl Iterator<Item = i32>) -> usize {
    values.filter(|value| *value > 0).map(|value| value + 1).count()
}

fn argument_chain(values: &[i32]) -> usize {
    (0..4)
        .zip(values.iter().filter(|value| **value > 0).map(|value| *value + 1).take(2))
        .count()
}

fn main() {}
