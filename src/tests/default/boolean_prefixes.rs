#![warn(prefer_boolean_prefixes)]

type Flag = bool;

fn parameters(active: bool, visible: &bool) {
    let _ = (active, visible);
}

fn bindings() {
    let active = true;
    let visible: Flag = false;
    let borrowed = &true;
    let (cached, count) = (false, 1);
    let ref writable = false;
    let r#match = true;
    let _ = (active, visible, borrowed, cached, count, writable, r#match);
}

fn alternatives(value: Result<bool, bool>) {
    match value {
        Ok(pending) | Err(pending) => {
            let _ = pending;
        }
    }
}

fn destructuring(value: Option<bool>) {
    if let Some(cached) = value {
        let _ = cached;
    }
}

fn closures() {
    let check = |active: bool| active;
    let _ = check;
}

fn boundaries(is: bool, has: bool, island: bool, isready: bool) {
    let _ = (is, has, island, isready);
}

fn main() {}
