/*
    Appellation: time <test>
    Contrib: @FL03
*/
use core::time::Duration;
use scsys_core::time::{Timestamp, systime};

fn absdiff<A, B, C>(a: A, b: B) -> C
where
    A: PartialOrd<B> + core::ops::Sub<B, Output = C>,
    B: core::ops::Sub<A, Output = C>,
{
    if a > b { a - b } else { b - a }
}

#[test]
fn test_timestamp() {
    let now = systime();
    let ts = Timestamp::<u128>::now();

    let tsd = Duration::from_millis(ts.0 as u64);
    assert_eq!(absdiff(now, tsd), Duration::from_millis(0));
}
