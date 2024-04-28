/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

pub trait AsSliceMut<T> {
    fn as_mut_slice(&mut self) -> &mut [T];
}

impl<S, T> AsSlice<T> for S
where
    S: AsRef<[T]>,
{
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<S, T> AsSliceMut<T> for S
where
    S: AsMut<[T]>,
{
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}
