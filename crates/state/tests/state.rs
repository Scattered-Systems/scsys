/*
    Appellation: state <test>
    Created At: 2025.09.08:19:05:49
    Contrib: @FL03
*/
use scsys_state::{NState, Nary, State};

#[test]
fn test_state() {
    let mut state = State::<usize>::zero();
    // verify the initial state is zero
    assert_eq!(state, 0);
    // set the state to a new value
    state.set(5);
    // verify the state is now 5
    assert_eq!(state, 5);
    // take the inner value leaving the default in its place
    assert_eq!(state.take(), 5);
    // verify the state is now back to its default value
    assert_eq!(state, 0);
    // map the state to a new value
    let mapped = state.map(|x| x + 1);
    assert_eq!(mapped, 1);
    // ensure the original state is unchanged
    assert_ne!(mapped, state);
}

#[test]
fn test_state_views() {
    let mut state = State::<usize>::zero();

    let view = state.view();
    // verify that the "view" contains a reference to the original value
    assert_eq!(view.get(), &&0);
    assert_eq!(view.copied(), 0);
    // set the state to a new value
    state.set(5);
    // verify that the "view_mut" contains a mutable reference to the original value
    assert_eq!(state.view().value(), &mut 5_usize);
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
    let state = NState::<usize, 4>::new(0);
    assert!(state.is_kind::<Nary<4>>());

    assert!(!state.is_kind::<Nary<2>>());
    assert!(!state.is_kind::<Nary<{ usize::MAX }>>());
}

#[cfg(feature = "rand")]
#[test]
fn test_random_state() {
    // generate some random state `a`
    let a = State::<f32>::random();
    // generate another random state `b`
    let b = State::<f32>::random();
    // ensure the two states are not equal
    assert_ne!(a, b);
}
