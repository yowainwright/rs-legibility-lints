fn at_limit(flag: bool) {
    if flag {}
    for _ in 0..1 {}
    while flag {
        break;
    }
    loop {
        break;
    }
    match flag {
        true => (),
        false => (),
    }
}

fn nested_loops(flag: bool) {
    for _ in 0..1 {
        while flag {
            break;
        }
    }
    loop {
        for _ in 0..1 {}
        break;
    }
}

fn branches(value: i32) {
    if value == 1 {
    } else if value == 2 {
        if value > 0 {}
    } else {
        if value < 0 {}
    }
}

fn match_arms(flag: bool) {
    match flag {
        true => if flag {},
        false => if !flag {},
    }
}

fn patterns(value: Option<i32>) {
    if let Some(value) = value {
        if value > 0 {}
    }
}

fn separate_bodies(flag: bool) {
    if flag {
        let check = || {
            if flag {}
        };
        check();
        let _ = async { if flag {} };
        fn nested(flag: bool) {
            if flag {}
        }
        nested(flag);
    }
}

fn nested_closure(flag: bool) {
    let check = || {
        if flag {
            if flag {}
        }
    };
    check();
}

fn main() {
    let flag = false;
    at_limit(flag);
    nested_loops(flag);
    branches(0);
    match_arms(flag);
    patterns(None);
    separate_bodies(flag);
    nested_closure(flag);
}
