#[allow(clippy::upper_case_acronyms)]
pub trait HKT<U> {
    type C; // Current Type
    type T; // Type C swapped with U
}

pub trait Functor<U>: HKT<U> {
    fn fmap<F>(self, f: F) -> Self::T
    where
        F: Fn(Self::C) -> U;
}

pub trait Applicative<U>: Functor<U> {
    fn pure_(value: U) -> Self::T
    where
        Self: HKT<U, C = U>;
    fn seq<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<U>>::T
    where
        F: Fn(&<Self as HKT<U>>::C) -> U,
        Self: HKT<F>;
}

pub trait Monad<U>: Applicative<U> {
    fn return_(x: U) -> Self::T
    where
        Self: HKT<U, C = U>,
    {
        Self::pure_(x)
    }

    fn bind<F>(&self, fs: F) -> Self::T
    where
        F: FnMut(&Self::C) -> Self::T;

    fn join<T>(&self) -> T
    where
        Self: HKT<U, T = T, C = T>,
        T: Clone,
    {
        self.bind(|x| x.clone())
    }
}
