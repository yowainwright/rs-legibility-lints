fn different_subject(value: u8, other: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if other == 3 {
    }
}

fn invalid_tail(value: u8, other: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 {
    } else if other == 4 {
    }
}

fn floats(value: f64) {
    if value == 1.0 {
    } else if value == 2.0 {
    } else if value == 3.0 {
    }
}

fn inequalities(value: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if value != 3 {
    }
}

fn fields(value: (u8,)) {
    if value.0 == 1 {
    } else if value.0 == 2 {
    } else if value.0 == 3 {
    }
}

fn calls(next: impl Fn() -> u8) {
    if next() == 1 {
    } else if next() == 2 {
    } else if next() == 3 {
    }
}

struct Custom(u8);

impl PartialEq<u8> for Custom {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

fn overloaded(value: Custom) {
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 {
    }
}

fn constants(value: u8) {
    const FIRST: u8 = 1;
    if value == FIRST {
    } else if value == 2 {
    } else if value == 3 {
    }
}

macro_rules! generated {
    ($value:expr) => {
        if $value == 1 {
        } else if $value == 2 {
        } else if $value == 3 {
        }
    };
}

macro_rules! literal {
    () => { 3 };
}

fn macro_condition(value: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if value == literal!() {
    }
}

#[allow(
    prefer_switch_over_long_if_chain,
    reason = "verify function-level lint suppression"
)]
fn suppressed(value: u8) {
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 {
    }
}

fn patterns(value: Option<u8>) {
    if let Some(1) = value {
    } else if let Some(2) = value {
    } else if let Some(3) = value {
    }
}

fn owned_strings(value: String) {
    if value == "new" {
    } else if value == "active" {
    } else if value == "closed" {
    }
}

#[allow(hoist_if_operators, reason = "isolate compound comparison exclusion")]
fn compound(value: u8, flag: bool) {
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 && flag {
    }
}

fn locally_suppressed(value: u8) {
    #[allow(
        prefer_switch_over_long_if_chain,
        reason = "verify expression-level lint suppression"
    )]
    if value == 1 {
    } else if value == 2 {
    } else if value == 3 {
    }
}

fn main() {
    generated!(0);
}
