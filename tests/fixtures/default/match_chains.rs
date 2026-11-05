fn below_limit(value: u8) {
    if value == 1 {
    } else if value == 2 {
    } else {
    }
}

fn at_limit(value: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 {
    }
}

fn above_limit(value: u8) -> &'static str {
    if value == 1 {
        "new"
    } else if value == 2 {
        "active"
    } else if value == 3 {
        "closed"
    } else if value == 4 {
        "archived"
    } else {
        "other"
    }
}

fn strings(value: &str) {
    if value == "new" {
    } else if "active" == value {
    } else if value == "closed" {
    }
}

fn characters(value: char) {
    if value == 'a' {
    } else if value == 'b' {
    } else if value == 'c' {
    }
}

fn negatives(value: i32) {
    if -1 == value {
    } else if value == -2 {
    } else if -3 == value {
    }
}

fn rewritten(value: u8) {
    match value {
        1 => (),
        2 => (),
        3 => (),
        _ => (),
    }
}

fn nested(value: u8, other: u8) {
    if value == 1 {
        if other == 1 {
        } else if other == 2 {
        } else if other == 3 {
        }
    } else if value == 2 {
    } else if value == 3 {
    }
}

fn main() {}
