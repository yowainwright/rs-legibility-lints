#[warn(require_filename_matches_dirname)]
#[path = "auxiliary/orders/ordersome.rs"]
mod warned;

#[deny(require_filename_matches_dirname)]
#[path = "auxiliary/orders/ordersome.rs"]
mod rejected;

fn main() {}
