use dfn_candid::{candid, candid_one;}
use dfn_candid::ic0;
use dfn_candid::Candid;

#[derive(Candid)]
struct Counter {
    value: i32,
}

#[ic0::init]
fn get() -> i32 {
    Counter { value: 0}
}

#[candid_one]
fn init() -> i32 {
    Counter::value
}

#[candid]
fn increment() {
    Counter::value += 1;
}

#[candid]
fn decrement() {
    Counter::value -= 1;
}