fn different_inputs(values: &[u32], other: &[u32], needle: u32, other_needle: u32) {
    let first = values.contains(&needle);
    let second = other.contains(&needle);
    let third = other.contains(&other_needle);
}

fn changed_collection(mut values: Vec<u32>, needle: u32) {
    let first = values.contains(&needle);
    values.push(needle);
    let second = values.contains(&needle);
}

fn changed_needle(values: &[u32], mut needle: u32) {
    let first = values.contains(&needle);
    needle += 1;
    let second = values.contains(&needle);
}

fn shadowed_needle(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    let needle = needle + 1;
    let second = values.contains(&needle);
}

fn intervening_statement(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    let number = 1;
    let second = values.contains(&needle);
}

struct Custom;

impl PartialEq for Custom {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

fn custom_equality(values: &[Custom], needle: Custom) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn custom_method() {
    struct Search;
    impl Search {
        fn contains(&self, _needle: &u32) -> bool {
            false
        }
    }
    let values = Search;
    let needle = 1;
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn calls(values: &[u32], next: impl Fn() -> u32) {
    let first = values.contains(&next());
    let second = values.contains(&next());
}

fn separate_blocks(values: &[u32], needle: u32) {
    { let first = values.contains(&needle); }
    { let second = values.contains(&needle); }
}

macro_rules! generated {
    ($values:expr, $needle:expr) => {
        let first = $values.contains(&$needle);
        let second = $values.contains(&$needle);
    };
}

fn macros(values: &[u32], needle: u32) {
    generated!(values, needle);
}

struct Wrapper {
    values: [u32; 2],
    reads: std::cell::Cell<usize>,
}

impl std::ops::Deref for Wrapper {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        self.reads.set(self.reads.get() + 1);
        &self.values
    }
}

fn custom_deref(values: &Wrapper, needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn other_collections(values: &std::collections::HashSet<u32>, needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn literals(values: &[u32]) {
    let first = values.contains(&1);
    let second = values.contains(&1);
}

#[allow(no_repeated_collection_search, reason = "verify function-level lint suppression")]
fn suppressed(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn locally_suppressed(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    #[allow(no_repeated_collection_search, reason = "verify binding-level lint suppression")]
    let second = values.contains(&needle);
}

fn main() {}
