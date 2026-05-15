/*
    Appellation: impl_state_ops <module>
    Created At: 2025.09.08:19:31:32
    Contrib: @FL03
*/
use crate::state::State;

use crate::traits::RawState;
use num_traits::{Num, One, Zero};

impl<Q> One for State<Q>
where
    Q: RawState + One,
{
    fn one() -> Self {
        State(Q::one())
    }
}

impl<Q> Zero for State<Q>
where
    Q: RawState + Zero,
{
    fn zero() -> Self {
        State(Q::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<Q> Num for State<Q>
where
    Q: RawState + Num,
{
    type FromStrRadixErr = Q::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Q::from_str_radix(s, radix).map(State)
    }
}

contained::binary_wrapper! {
    impl State {
        Add.add,
        Div.div,
        Mul.mul,
        Sub.sub,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr,
    }
}

contained::unary_wrapper! {
    impl State {
        Neg.neg,
        Not.not,
    }
}
