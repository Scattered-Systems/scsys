/*
    appellation: impl_id_ops <module>
    authors: @FL03
*/
use crate::id::Id;
use num_traits::{Num, One, Zero};

impl<T> One for Id<T>
where
    T: One,
{
    fn one() -> Self {
        Id(T::one())
    }
}

impl<T> Zero for Id<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Id(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<T> Num for Id<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(s, radix).map(Id)
    }
}

contained::binary_wrapper! {
    impl Id {
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr
    }
}

contained::unary_wrapper! {
    impl Id {
        Neg.neg,
        Not.not
    }
}
