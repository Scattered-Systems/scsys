/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::states::StateKind;
use strum::{EnumCount, EnumIs, VariantNames};

/// An enumerated type representing the possible binary states of a particular value.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    EnumCount,
    strum::EnumDiscriminants,
    EnumIs,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[repr(C)]
#[strum_discriminants(
    name(BinaryState),
    derive(
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        EnumCount,
        EnumIs,
        strum::EnumIter,
        strum::EnumString,
        VariantNames
    )
)]
#[strum(serialize_all = "lowercase")]
pub enum BinState<Q = ()> {
    Invalid(Q),
    Valid(Q),
}

impl<Q> BinState<Q> {
    pub fn from_bool(valid: bool, state: Q) -> Self {
        if valid {
            Self::Valid(state)
        } else {
            Self::Invalid(state)
        }
    }

    pub fn invalid(state: Q) -> Self {
        Self::Invalid(state)
    }

    pub fn valid(state: Q) -> Self {
        Self::Valid(state)
    }

    pub fn kind(&self) -> &str {
        match self {
            Self::Invalid(_) => "invalid",
            Self::Valid(_) => "valid",
        }
    }

    pub fn into_inner(self) -> Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }

    pub fn invalidate(self) -> Self {
        Self::Invalid(self.into_inner())
    }

    pub fn validate(self) -> Self {
        Self::Valid(self.into_inner())
    }

    pub fn update<Q2>(&mut self, state: Q2) -> BinState<Q2> {
        match self {
            Self::Invalid(_) => BinState::Invalid(state),
            Self::Valid(_) => BinState::Valid(state),
        }
    }
}

impl BinaryState {
    pub fn new(state: u8) -> Self {
        match state % Self::COUNT as u8 {
            0 => Self::Invalid,
            1 => Self::Valid,
            _ => panic!("Invalid binary state: {}", state),
        }
    }

    pub fn invalid() -> Self {
        Self::Invalid
    }

    pub fn valid() -> Self {
        Self::Valid
    }
}

impl<Q> AsRef<Q> for BinState<Q> {
    fn as_ref(&self) -> &Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q> AsMut<Q> for BinState<Q> {
    fn as_mut(&mut self) -> &mut Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q: Default> Default for BinState<Q> {
    fn default() -> Self {
        Self::Invalid(Q::default())
    }
}

impl Default for BinaryState {
    fn default() -> Self {
        Self::Invalid
    }
}

impl<Q> core::ops::Deref for BinState<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<Q> core::ops::DerefMut for BinState<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<Q> core::fmt::Display for BinState<Q>
where
    Q: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", *self)
    }
}

/*
    ************* Implementations *************
*/
impl StateKind for BinState {}
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
