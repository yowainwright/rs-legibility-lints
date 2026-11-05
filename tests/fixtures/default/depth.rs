fn at_limit(flag: bool) {
    if flag {
        for _ in 0..1 {
            if flag {}
        }
    }
}

fn above_limit(flag: bool) {
    if flag {
        for _ in 0..1 {
            if flag {
                while flag {
                    break;
                }
            }
        }
    }
}

fn deeper(flag: bool) {
    if flag {
        if flag {
            if flag {
                if flag {
                    if flag {}
                }
            }
        }
    }
}

fn match_depth(flag: bool) {
    if flag {
        loop {
            match flag {
                true => if flag {},
                false => (),
            }
            break;
        }
    }
}

fn else_if_chain(value: i32) {
    if value > 0 {
        loop {
            if value == 1 {
            } else if value == 2 {
            } else if value == 3 {
            } else {
            }
            break;
        }
    }
}

fn plain_blocks(flag: bool) {
    { { { { if flag {} } } } }
}

async fn await_at_limit(flag: bool) {
    if flag {
        if flag {
            if flag {
                std::future::ready(()).await;
            }
        }
    }
}

fn try_at_limit(flag: bool) -> Option<()> {
    if flag {
        if flag {
            if flag {
                Some(())?;
            }
        }
    }
    Some(())
}

#[allow(
    max_control_flow_depth,
    reason = "verify function-level lint suppression"
)]
fn suppressed(flag: bool) {
    if flag {
        if flag {
            if flag {
                if flag {}
            }
        }
    }
}

fn locally_suppressed(flag: bool) {
    if flag {
        if flag {
            if flag {
                #[allow(
                    max_control_flow_depth,
                    reason = "verify expression-level lint suppression"
                )]
                if flag {}
            }
        }
    }
}

macro_rules! generated {
    ($flag:expr) => {
        for _ in 0..1 {
            if $flag {
                while $flag {
                    if $flag {}
                    break;
                }
            }
        }
    };
}

fn main() {
    let flag = false;
    at_limit(flag);
    above_limit(flag);
    deeper(flag);
    match_depth(flag);
    else_if_chain(0);
    plain_blocks(flag);
    let _ = await_at_limit(flag);
    let _ = try_at_limit(flag);
    suppressed(flag);
    locally_suppressed(flag);
    generated!(false);
}
