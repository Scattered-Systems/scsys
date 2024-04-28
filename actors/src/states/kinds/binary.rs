/*
    Appellation: binary <kinds>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::states::StateKind;
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryState {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl BinaryState {
    pub fn new(state: u8) -> Self {
        match state {
            1 => Self::Valid,
            _ => Self::Invalid,
        }
    }
    /// a functional variant constructor for [Invalid](BinaryState::Invalid)
    pub fn invalid() -> Self {
        Self::Invalid
    }
    /// a functional variant constructor for [Valid](BinaryState::Valid)
    pub fn valid() -> Self {
        Self::Valid
    }

    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}

impl StateKind for BinaryState {}

macro_rules! impl_std_unary {
    ($($($p:ident)::*.$call:ident),*) => {
        $(impl_std_unary!(@impl $($p)::*.$call);)*
    };
    (std $($trait:ident.$call:ident),*) => {
        $(impl_std_unary!(@impl core::ops::$trait.$call);)*
    };
    (@impl $($p:ident)::*.$call:ident) => {
        impl $($p)::* for BinaryState {
            type Output = BinaryState;

            fn $call(self) -> Self::Output {
                BinaryState::new($($p)::*::$call(self as u8))
            }
        }
    }
}

macro_rules! impl_std_binary {
    ($($($p:ident)::*.$call:ident),*) => {
        $(impl_std_binary!(@impl $($p)::*.$call);)*
    };
    (std $($trait:ident.$call:ident),*) => {
        $(
            impl_std_binary!(@assign $trait.$call);
            impl_std_binary!(@impl core::ops::$trait.$call);
        )*
    };
    (@impl $($p:ident)::*.$call:ident) => {
        impl_std_binary!(@self $($p)::*.$call);
        impl_std_binary!(@primitive $($p)::*.$call);
    };

    (@self $($p:ident)::*.$call:ident) => {
        impl $($p)::* for BinaryState {
            type Output = BinaryState;

            fn $call(self, rhs: BinaryState) -> Self::Output {
                let res = $($p)::*::$call(self as u8, rhs as u8);
                BinaryState::new(res)
            }
        }

        impl<'a> $($p)::* for &'a BinaryState {
            type Output = BinaryState;

            fn $call(self, rhs: &'a BinaryState) -> Self::Output {
                let res = $($p)::*::$call(*self as u8, *rhs as u8);
                BinaryState::new(res)
            }
        }

        impl<'a> $($p)::*<BinaryState> for &'a BinaryState {
            type Output = BinaryState;

            fn $call(self, rhs: BinaryState) -> Self::Output {
                let res = $($p)::*::$call(*self as u8, rhs as u8);
                BinaryState::new(res)
            }
        }

        impl<'a> $($p)::*<&'a BinaryState> for BinaryState {
            type Output = BinaryState;

            fn $call(self, rhs: &'a BinaryState) -> Self::Output {
                let res = $($p)::*::$call(self as u8, *rhs as u8);
                BinaryState::new(res)
            }
        }
    };
    (@assign $trait:ident.$call:ident) => {
        paste::paste! {
            impl core::ops::[<$trait Assign>] for BinaryState {
                fn [<$call _assign>](&mut self, rhs: BinaryState) {
                    *self = core::ops::$trait::$call(*self, rhs);
                }
            }

            impl<'a> core::ops::[<$trait Assign>]<&'a BinaryState> for BinaryState {
                fn [<$call _assign>](&mut self, rhs: &'a BinaryState) {
                    *self = core::ops::$trait::$call(*self, rhs);
                }
            }
        }
    };

    (@primitive $($p:ident)::*.$call:ident) => {
        impl<T> $($p)::*<T> for BinaryState where u8: $($p)::*<T, Output = u8> {
            type Output = BinaryState;

            fn $call(self, rhs: T) -> Self::Output {
                let res = $($p)::*::$call(self as u8, rhs);
                BinaryState::new(res)
            }
        }

        impl<'a, T> $($p)::*<T> for &'a BinaryState where u8: $($p)::*<T, Output = u8> {
            type Output = BinaryState;

            fn $call(self, rhs: T) -> Self::Output {
                let res = $($p)::*::$call(*self as u8, rhs);
                BinaryState::new(res)
            }
        }
    };

}

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            enum_from_primitive!(BinaryState(0: Invalid, 1: Valid)<$t>);
        )*
    };
}

impl_std_binary!(std Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);
impl_std_binary!(std BitAnd.bitand, BitOr.bitor, BitXor.bitxor, Shl.shl, Shr.shr);
impl_std_unary!(core::ops::Not.not);

impl_from!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
