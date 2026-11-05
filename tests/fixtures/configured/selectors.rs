fn at_limit(value: &((u8,),)) -> &(u8,) {
    &value.0
}

fn above_limit(value: &((u8,),)) -> u8 {
    value.0.0
}

fn main() {}
