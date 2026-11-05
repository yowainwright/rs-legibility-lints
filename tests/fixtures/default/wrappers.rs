fn clean_name(name: &str) -> &str {
    name.trim()
}

fn returned(value: u8) -> u8 {
    return target(value);
}

fn tail_return(value: u8) -> u8 {
    return target(value)
}

fn multiple(left: u8, right: u8) -> u8 {
    pair(left, right)
}

fn unit_tail(values: &mut Vec<u8>) {
    values.clear()
}

fn unit_statement(values: &mut Vec<u8>) {
    values.clear();
}

fn qualified(value: u8) -> u8 {
    arithmetic::target(value)
}

fn associated_call(name: &str) -> &str {
    str::trim(name)
}

use arithmetic::target as renamed_target;

fn alias(value: u8) -> u8 {
    renamed_target(value)
}

fn mutable(mut value: u8) -> u8 {
    target(value)
}

struct State(u8);

impl State {
    fn forwarded(&self, value: u8) -> u8 {
        self.convert(value)
    }

    fn associated(value: u8) -> u8 {
        target(value)
    }

    fn convert(&self, value: u8) -> u8 {
        self.0.wrapping_add(value)
    }
}

mod arithmetic {
    pub fn target(value: u8) -> u8 {
        value.wrapping_add(1)
    }
}

fn target(value: u8) -> u8 {
    value.wrapping_add(1)
}

pub fn pair(left: u8, right: u8) -> u8 {
    left.wrapping_add(right)
}

fn main() {}
