/*
    appellation: raw_state <module>
    authors: @FL03
*/

/// [RawState] is a trait that defines the types of states
pub trait RawState {
    private!();
}

/*
 ************* Implementations *************
*/

impl<Q> RawState for &Q
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for &mut Q
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for [Q]
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for &[Q]
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for &mut [Q]
where
    Q: RawState,
{
    seal!();
}

macro_rules! impl_raw_state {
    (@impl $($t:ident)::*<$T:ident>) => {
        impl<$T> $crate::state::RawState for $($t)::*<$T>
        where
            $T: $crate::state::RawState,
        {
            seal!();
        }
    };
    (@prim $t:ty) => {
        impl $crate::state::RawState for $t {
            seal!();
        }
    };
    ($($($t:ident)::*<$T:ident> ),* $(,)?) => {
        $(
            impl_raw_state!(@impl $($t)::*<$T>);
        )*
    };
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_state!(@prim $t);
        )*
    };
}

impl_raw_state! {
    Option<Q>,
    core::mem::MaybeUninit<Q>,
    core::marker::PhantomData<Q>,
}

impl_raw_state! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64, bool, char, str,
}

#[cfg(feature = "alloc")]
impl_raw_state! {
    alloc::string::String
}

#[cfg(feature = "alloc")]
impl_raw_state! {
    alloc::sync::Arc<Q>,
    alloc::boxed::Box<Q>,
    alloc::rc::Rc<Q>,
    alloc::vec::Vec<Q>,
}

#[cfg(feature = "std")]
impl_raw_state! {
    std::cell::Cell<Q>,
    std::sync::Mutex<Q>,
}
