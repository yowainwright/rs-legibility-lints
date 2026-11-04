type Flag = bool;

struct State {
    is_not_ready: bool,
}

struct Custom(bool);

fn locals(status: &str) {
    let is_not_ready = status != "ready";
    let has_no_owner = false;
    let can_not_send: Flag = false;
    let does_not_exist = &false;
    if is_not_ready {}
    let _ = (has_no_owner, can_not_send, does_not_exist);
}

fn parameter(should_not_retry: bool) {
    if should_not_retry {}
}

fn patterns(value: Option<bool>) {
    let (are_not_ready, number) = (false, 1);
    let Some(was_not_ready) = value else {
        return;
    };
    let ref were_not_ready = false;
    let _ = (are_not_ready, number, was_not_ready, were_not_ready);
}

fn alternatives(value: Result<bool, bool>) {
    match value {
        Ok(have_no_value) | Err(have_no_value) => {
            let _ = have_no_value;
        }
    }
}

fn closures(value: bool) {
    let check = |had_no_value: bool| had_no_value;
    let _ = check(value);
}

fn conditional_bindings(value: Option<bool>) {
    while let Some(could_not_finish) = value {
        let _ = could_not_finish;
        break;
    }
    if let Some(will_not_finish) = value {
        let _ = will_not_finish;
    }
}

fn flag() -> bool {
    false
}

fn inferred_and_raw() {
    let r#would_not_cache = false;
    let did_not_run = flag();
    let _ = (r#would_not_cache, did_not_run);
}

fn accepted(status: &str) {
    let is_ready = status == "ready";
    let is_noteworthy = true;
    let island_not_ready = true;
    let no_owner = false;
    let _is_not_ready = true;
    let is_not_ = true;
    let _ = (is_ready, is_noteworthy, island_not_ready, no_owner, is_not_);
}

fn other_types(value: bool) {
    let is_not_count = 1;
    let has_no_message = "message";
    let can_not_flag = Custom(value);
    let _ = (is_not_count, has_no_message, can_not_flag);
    let state = State {
        is_not_ready: false,
    };
    if state.is_not_ready {}
}

#[allow(
    prefer_positive_condition_names,
    reason = "verify function-level lint suppression"
)]
fn suppressed(is_not_argument: bool) {
    let has_no_limit = is_not_argument;
    let _ = has_no_limit;
}

fn locally_suppressed() {
    #[allow(
        prefer_positive_condition_names,
        reason = "verify binding-level lint suppression"
    )]
    let is_not_reported = false;
    let _ = is_not_reported;
}

macro_rules! generated {
    () => {
        let is_not_generated = false;
        let _ = is_not_generated;
    };
}

fn main() {
    let flag = false;
    locals("ready");
    parameter(flag);
    patterns(Some(flag));
    alternatives(Ok(flag));
    closures(flag);
    conditional_bindings(Some(flag));
    inferred_and_raw();
    accepted("ready");
    other_types(flag);
    suppressed(flag);
    locally_suppressed();
    generated!();
}
