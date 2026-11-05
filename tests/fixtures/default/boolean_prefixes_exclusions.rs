#![warn(prefer_boolean_prefixes)]

fn prefixes() {
    let (is_ready, are_ready, was_ready, were_ready) = (true, true, true, true);
    let (has_owner, have_owners, had_owner) = (true, true, true);
    let (can_send, could_send, should_send) = (true, true, true);
    let (will_run, would_run, did_run, does_run) = (true, true, true, true);
}

fn exceptions() {
    let (debug, disabled, done, empty, enabled, exists) = (true, true, true, true, true, true);
    let (found, invalid, ok, ready, valid) = (true, true, true, true, true);
    let _active = true;
    let _ = false;
}

struct Flag(bool);

struct State {
    active: bool,
}

const ACTIVE: bool = true;
static VISIBLE: bool = false;

fn other_types() {
    let active = 1;
    let visible = "visible";
    let flags = [true];
    let wrapped = Flag(ACTIVE);
    let state = State { active: ACTIVE };
}

trait Check {
    fn check(self);
}

impl Check for bool {
    fn check(self) {}
}

macro_rules! generated {
    () => {
        let active = true;
    };
}

fn macros() {
    generated!();
}

#[allow(prefer_boolean_prefixes, reason = "verify function-level lint suppression")]
fn suppressed(active: bool) {
    let visible = active;
}

fn locally_suppressed() {
    #[allow(prefer_boolean_prefixes, reason = "verify binding-level lint suppression")]
    let active = true;
}

fn main() {}
