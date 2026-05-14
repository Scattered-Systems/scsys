/*
    appellation: raw_state <module>
    authors: @FL03
*/

/// [RawState] is a trait that defines the types of states
pub trait RawState: Send + Sync + core::fmt::Debug {
    private!();
}

/*
 ************* Implementations *************
*/
impl<Q> RawState for Q
where
    Q: Send + Sync + core::fmt::Debug,
{
    seal!();
}

#[allow(unused_macros)]
macro_rules! impl_raw_state {
    (@impl $t:ty) => {
        impl $crate::state::RawState for $t {
            seal!();
        }
    };
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_state!(@impl $t);
        )*
    };
}

// impl_raw_state! {
//     u8, u16, u32, u64, u128, usize,
//     i8, i16, i32, i64, i128, isize,
//     f32, f64, bool, char, str,
// }

// impl<Q> RawState for &Q where Q: RawState {
//     seal!();
// }

// impl<Q> RawState for &mut Q where Q: RawState {
//     seal!();
// }

// impl<Q> RawState for [Q] where Q: RawState {
//     seal!();
// }

// impl<Q> RawState for &[Q] where Q: RawState {
//     seal!();
// }

// impl<Q> RawState for &mut [Q] where Q: RawState {
//     seal!();
// }
