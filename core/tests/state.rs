/*
    Appellation: state <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys_core::state::*;

#[test]
fn test_nary_state() {
    let state = NaryState::<usize, 4>::new(0);
    assert!(state.is_state::<Nary<4>>());

    assert!(!state.is_state::<Nary<2>>());
    assert!(!state.is_state::<Nary<{ usize::MAX }>>());
}
