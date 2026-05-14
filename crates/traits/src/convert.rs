/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// [`AsSlice`] is a generic trait for converting a type into a slice of its
/// elements. This is useful for types that can be represented as a contiguous sequence of
/// elements, such as arrays, vectors, or other collections.
pub trait AsSlice<T> {
    private!();

    fn as_slice(&self) -> &[T];
}
/// [`AsSliceMut`] is a generic trait for converting a type into a mutable slice of its
/// elements.
pub trait AsSliceMut<T> {
    private!();
    /// converts the type into a mutable slice of its elements.
    fn as_mut_slice(&mut self) -> &mut [T];
}

/*
 ************* Implementations *************
*/
impl<S, T> AsSlice<T> for S
where
    S: AsRef<[T]>,
{
    seal!();

    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<S, T> AsSliceMut<T> for S
where
    S: AsMut<[T]>,
{
    seal!();

    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}
