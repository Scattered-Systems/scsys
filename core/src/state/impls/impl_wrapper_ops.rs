/*
    Appellation: impl_ops <module>
    Contrib: @FL03
*/
use crate::state::State;

macro_rules! impl_binary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_binary_op!(@impl $s::$op.$call);
            impl_binary_op!(@mut $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B, C> ::core::ops::$op<$s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, &rhs.0))
            }
        }
    };
    (@mut $s:ident::$op:ident.$call:ident) => {
        paste::paste! {
            impl_binary_op_mut!(@impl $s::[<$op Assign>].[<$call _assign>]);
        }
    };
}

macro_rules! impl_binary_op_mut {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_binary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op<$s<B>> for &mut $s<A>
        where
            A: ::core::ops::$op<B>,
        {

            fn $call(&mut self, rhs: $s<B>) {
                core::ops::$op::$call(&mut self.0, rhs.0)
            }
        }
    };
}

macro_rules! impl_unary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_unary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A> ::core::ops::$op for &mut $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }

        impl<'a, A> ::core::ops::$op for &mut &'a $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }

        impl<'a, A> ::core::ops::$op for &mut &'a mut $s<A>
        where
            A: Clone + ::core::ops::$op,
        {
            type Output = $s<A::Output>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0.clone()))
            }
        }
    };
}

impl_binary_op! {
    State::<[
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
    ]>
}

impl_unary_op! {
    State::<[
        Neg.neg,
        Not.not
    ]>
}
