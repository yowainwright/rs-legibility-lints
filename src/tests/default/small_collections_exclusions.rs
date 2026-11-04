use std::borrow::Borrow;
use std::collections::{BTreeSet, HashMap, HashSet};

fn at_limit(value: u8) -> bool {
    HashSet::from([1, 2, 3]).contains(&value)
}

fn above_limit(value: u8) -> bool {
    HashSet::from([1, 2, 3, 4]).contains(&value)
}

fn named_set(value: u8) -> bool {
    let allowed = HashSet::from([1, 2]);
    allowed.contains(&value)
}

fn named_input(value: u8) -> bool {
    let values = [1, 2];
    HashSet::from(values).contains(&value)
}

fn dynamic_input(values: [u8; 2], value: u8) -> bool {
    HashSet::from(values).contains(&value)
}

fn nonliteral_items(left: u8, right: u8, value: u8) -> bool {
    HashSet::from([left, right]).contains(&value)
}

fn collected(value: u8) -> bool {
    [1, 2].into_iter().collect::<HashSet<_>>().contains(&value)
}

fn tree(value: u8) -> bool {
    BTreeSet::from([1, 2]).contains(&value)
}

#[allow(no_deep_composite_literal_arg, reason = "isolate the HashMap exclusion")]
fn map(value: u8) -> bool {
    HashMap::from([(1, 2), (3, 4)]).contains_key(&value)
}

fn other_operation(value: u8) -> bool {
    HashSet::from([1, 2]).get(&value).is_some()
}

#[derive(Eq, Hash, PartialEq)]
struct Custom(u8);

fn custom_items(value: &Custom) -> bool {
    HashSet::from([Custom(1), Custom(2)]).contains(value)
}

fn empty_custom(value: &Custom) -> bool {
    HashSet::<Custom>::from([]).contains(value)
}

impl Borrow<Custom> for u8 {
    fn borrow(&self) -> &Custom {
        static VALUE: Custom = Custom(1);
        &VALUE
    }
}

fn custom_query(value: &Custom) -> bool {
    HashSet::from([1_u8, 2]).contains(value)
}

struct CustomSet;

impl CustomSet {
    fn from(_: [u8; 2]) -> Self {
        Self
    }

    fn contains(&self, _: &u8) -> bool {
        false
    }
}

fn unrelated(value: u8) -> bool {
    CustomSet::from([1, 2]).contains(&value)
}

mod custom_conversion {
    use std::collections::HashSet;

    trait Factory {
        fn from(values: [u8; 2]) -> Self;
    }

    impl Factory for HashSet<u8> {
        fn from(_: [u8; 2]) -> Self {
            Self::new()
        }
    }

    pub fn lookup(value: u8) -> bool {
        <HashSet<u8> as Factory>::from([1, 2]).contains(&value)
    }
}

macro_rules! values {
    () => { [1, 2] };
}

macro_rules! generated {
    ($value:expr) => {
        HashSet::from([1, 2]).contains(&$value)
    };
}

fn macro_input(value: u8) -> bool {
    HashSet::from(values!()).contains(&value)
}

#[allow(no_small_collection_conversion, reason = "verify native lint suppression")]
fn suppressed(value: u8) -> bool {
    HashSet::from([1, 2]).contains(&value)
}

fn accepted(status: &str) -> bool {
    ["done", "failed"].contains(&status)
}

fn main() {
    let _ = generated!(0);
}
