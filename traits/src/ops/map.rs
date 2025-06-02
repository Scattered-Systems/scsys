/*
    appellation: map <module>
    authors: @FL03
*/
/// [`Map`] is a trait that allows for mapping over a reference to a type `T` by applying a 
/// function [`F`](FnOnce) to it, producing a new type `U`. The result is wrapped in a
/// container type defined by the implementor of the trait. 
pub trait Map<T> {
    /// The associated type `Cont` is a higher-kinded type that represents the container
    /// that will hold the result of the mapping operation.
    type Cont<_T: ?Sized>;

    fn map<U, F>(&self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(&T) -> U;
}
/// A the [`MapOnce`] trait is similar to [`Map`], but it consumes the instance instead of 
/// borrowing it; 
pub trait MapOnce<T> {
    type Cont<_T: ?Sized>;

    fn mapc<U, F>(self, rhs: F) -> Self::Cont<U>
    where
        F: FnOnce(T) -> U;
}

pub trait MapInplace<T> {
    type Cont<_T: ?Sized>;

    fn map_inplace<F>(&mut self, rhs: F) -> &mut Self::Cont<T>
    where
        F: FnMut(&mut T);
}
