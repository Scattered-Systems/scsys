/*
    Appellation: state <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys_core as scsys;

use scsys::state::{NState, Nary, State};

#[test]
fn test_option_state() {
    let mut state = State::<Option<usize>>::none();
    assert_eq!(state.get(), &None);
    state.set(Some(5));
    assert_eq!(state.get(), &Some(5));
}

#[test]
fn test_nary_state() {
    let state = NState::<usize, 4>::new(0);
    assert!(state.is_kind::<Nary<4>>());

    assert!(!state.is_kind::<Nary<2>>());
    assert!(!state.is_kind::<Nary<{ usize::MAX }>>());
}
