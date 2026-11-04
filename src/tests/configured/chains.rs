fn at_limit(values: &[i32]) -> usize {
    values.iter().count()
}

fn above_limit(values: &[i32]) -> Vec<i32> {
    values.iter().map(|value| *value + 1).collect()
}

fn main() {}
