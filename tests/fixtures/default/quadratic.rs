fn nested_for(values: &[u32]) {
    for left in values {
        for right in values {
            let _ = (left, right);
        }
    }
}

fn nested_loops(flag: bool) {
    while flag {
        for _ in 0..1 {}
        loop {
            break;
        }
        break;
    }
}

fn three_loops() {
    for _ in 0..1 {
        for _ in 0..1 {
            for _ in 0..1 {}
        }
    }
}

fn linear_searches(slice: &[u32], vector: &Vec<u32>, array: &[u32; 3], needles: &[u32]) {
    for id in needles {
        if slice.contains(id) {}
        if vector.contains(id) {}
        if array.contains(id) {}
    }
}

fn search_in_while(values: &[u32], id: u32) {
    while values.contains(&id) {
        break;
    }
}

fn indexed_search(values: &[u32], needles: &[u32]) {
    let indexed: std::collections::HashSet<_> = values.iter().copied().collect();
    for id in needles {
        if indexed.contains(id) {}
    }
}

fn other_lookups(needles: &[u32]) {
    let tree = std::collections::BTreeSet::<u32>::new();
    let map = std::collections::HashMap::<u32, u32>::new();
    let deque = std::collections::VecDeque::<u32>::new();
    for id in needles {
        let _ = tree.contains(id);
        let _ = map.contains_key(id);
        let _ = deque.contains(id);
        let _ = "value".contains("v");
    }
}

struct Custom;

impl Custom {
    fn contains(&self, _value: &u32) -> bool {
        false
    }
}

fn custom_method(needles: &[u32]) {
    for id in needles {
        let _ = Custom.contains(id);
    }
}

fn sequential_loops(values: &[u32]) {
    let _ = values.contains(&1);
    for value in values {
        let _ = value;
    }
    for value in values {
        let _ = value;
    }
}

fn separate_bodies(values: &[u32]) {
    for _ in values {
        let check = || {
            for _ in values {}
        };
        check();
        let _ = async { for _ in values {} };
        fn nested(values: &[u32]) {
            for _ in values {}
        }
        nested(values);
    }
}

fn nested_closure(values: &[u32]) {
    let check = || {
        for _ in values {
            for _ in values {}
        }
    };
    check();
}

fn iterator_search(values: &[u32], needles: &[u32]) {
    for id in needles {
        let _ = values.iter().any(|value| value == id);
    }
}

#[allow(
    no_quadratic_patterns,
    reason = "verify function-level lint suppression"
)]
fn suppressed(values: &[u32]) {
    for id in values {
        for _ in values {}
        let _ = values.contains(id);
    }
}

fn locally_suppressed(values: &[u32]) {
    for id in values {
        #[allow(no_quadratic_patterns, reason = "verify loop-level lint suppression")]
        for _ in values {}
        #[allow(no_quadratic_patterns, reason = "verify call-level lint suppression")]
        let _ = values.contains(id);
    }
}

macro_rules! generated {
    ($values:expr) => {
        for id in $values {
            for _ in $values {}
            let _ = $values.contains(id);
        }
    };
}

mod custom_trait {
    struct CustomResult;

    trait CustomContains {
        fn contains(&self, value: &u32) -> CustomResult;
    }

    impl CustomContains for &[u32] {
        fn contains(&self, _value: &u32) -> CustomResult {
            CustomResult
        }
    }

    pub fn check(values: &&[u32], needles: &[u32]) {
        for id in needles {
            let _: CustomResult = values.contains(id);
        }
    }
}

fn main() {
    let values = vec![1, 2, 3];
    let flag = false;
    nested_for(&values);
    nested_loops(flag);
    three_loops();
    linear_searches(&values, &values, &[1, 2, 3], &values);
    search_in_while(&values, 1);
    indexed_search(&values, &values);
    other_lookups(&values);
    custom_method(&values);
    custom_trait::check(&&values[..], &values);
    sequential_loops(&values);
    separate_bodies(&values);
    nested_closure(&values);
    iterator_search(&values, &values);
    suppressed(&values);
    locally_suppressed(&values);
    generated!(&values);
}
