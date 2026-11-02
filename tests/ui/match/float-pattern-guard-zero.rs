//! Regression test for https://github.com/rust-lang/rust/issues/165311.
//! Regression from https://github.com/rust-lang/rust/pull/122459.

//@ run-pass

#![forbid(unsafe_code)]

#[inline(never)]
fn f(x: f64, b: bool) -> u8 {
    match x {
        0.0 if b => 1,
        -0.0 => 2,
        _ => 3,
    }
}

fn main() {
    assert_eq!(f(0.0, false), 2);
    assert_eq!(f(-0.0, false), 2);
    assert_eq!(f(0.0, true), 1);
    assert_eq!(f(-0.0, true), 1);
    assert_eq!(f(1.0, false), 3);
    assert_eq!(f(-1.0, true), 3);
}
