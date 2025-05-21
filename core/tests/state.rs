/*
    Appellation: state <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys_core::state::State;
use scsys_core::state::nstate::{Nary, NaryState};

#[test]
fn test_state() {
    let mut state = State::<usize>::new();
    assert_eq!(state.get(), &0);
    state.set(5);
    assert_eq!(state.get(), &5);
    assert_eq!(state.take(), 5);
    assert_eq!(state.get(), &0);
    assert_eq!(state.map(|x| x + 1), State(1));
}

#[test]
fn test_option_state() {
    let mut state = State::<Option<usize>>::none();
    assert_eq!(state.get(), &None);
    state.set(Some(5));
    assert_eq!(state.get(), &Some(5));
}

#[test]
fn test_nary_state() {
    let state = NaryState::<usize, 4>::new(0);
    assert!(state.is_state::<Nary<4>>());

    assert!(!state.is_state::<Nary<2>>());
    assert!(!state.is_state::<Nary<{ usize::MAX }>>());
}
