#![warn(require_filename_matches_dirname)]

#[allow(require_filename_matches_dirname, reason = "verify a later module can still warn")]
#[path = "auxiliary/orders/service.rs"]
mod suppressed;

#[path = "auxiliary/orders/service.rs"]
mod service;

#[path = "auxiliary/orders/service.rs"]
mod repeated;

#[path = "auxiliary/orders/orders.rs"]
mod exact;

#[path = "auxiliary/orders/orders_service.rs"]
mod prefixed;

#[path = "auxiliary/orders/lib.rs"]
mod library;

#[path = "auxiliary/orders/main.rs"]
mod binary;

#[path = "auxiliary/orders/mod.rs"]
mod directory;

#[cfg(any())]
#[path = "auxiliary/orders/missing.rs"]
mod unused;

fn main() {}
