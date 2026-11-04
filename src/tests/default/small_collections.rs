use std::collections::HashSet;

fn terminal(status: &str) -> bool {
    HashSet::from(["done", "failed"]).contains(&status)
}

fn single(value: u8) -> bool {
    HashSet::from([1]).contains(&value)
}

fn empty(value: u8) -> bool {
    HashSet::<u8>::from([]).contains(&value)
}

fn negative(value: i32) -> bool {
    HashSet::from([-1, -2]).contains(&value)
}

fn characters(value: char) -> bool {
    HashSet::from(['a', 'b']).contains(&value)
}

fn booleans(value: bool) -> bool {
    HashSet::from([true, false]).contains(&value)
}

type Allowed = HashSet<u8>;

fn alias(value: u8) -> bool {
    Allowed::from([1, 2]).contains(&value)
}

fn qualified(value: u8) -> bool {
    <HashSet<u8> as From<[u8; 2]>>::from([1, 2]).contains(&value)
}

fn main() {}
