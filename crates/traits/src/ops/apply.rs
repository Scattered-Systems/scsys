/*
    appellation: map <module>
    authors: @FL03
*/
/// [`Apply`] is a trait that allows for mapping over a reference to a type `T` by applying a
/// function [`F`](FnOnce) to it, producing a new type `U`. The result is wrapped in a
/// container type defined by the implementor of the trait.
pub trait Apply<T> {
    /// The associated type `Cont` is a higher-kinded type that represents the container
    /// that will hold the result of the mapping operation.
    type Cont<_T>;

    fn apply<U, F>(&self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(&T) -> U;
}
/// A the [`ApplyOnce`] trait is similar to [`Map`], but it consumes the instance instead of
/// borrowing it;
pub trait ApplyOnce<T> {
    type Cont<_T>;

    fn apply_once<U, F>(self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(T) -> U;
}
/// The [`ApplyMut`] trait allows for in-place mapping of a mutable reference to a type `T`
pub trait ApplyMut<T> {
    type Cont<_T>;

    fn apply_mut<F>(&mut self, rhs: F) -> &mut Self::Cont<T>
    where
        F: FnMut(&mut T);
}

/*
 ************* Implementations *************
*/

impl<T> Apply<T> for Option<T> {
    type Cont<U> = Option<U>;

    fn apply<U, F>(&self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_ref().map(rhs)
    }
}

impl<T> ApplyOnce<T> for Option<T> {
    type Cont<U> = Option<U>;

    fn apply_once<U, F>(self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(T) -> U,
    {
        self.map(rhs)
    }
}

impl<T> ApplyMut<T> for Option<T> {
    type Cont<U> = Option<U>;

    fn apply_mut<F>(&mut self, mut rhs: F) -> &mut Self::Cont<T>
    where
        F: FnMut(&mut T),
    {
        if let Some(value) = self {
            rhs(value);
        }
        self
    }
}
