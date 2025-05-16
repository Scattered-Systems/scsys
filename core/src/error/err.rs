pub struct Error {
    err: Box<dyn core::error::Error + Send + Sync + 'static>,
}

impl Error {
    /// Creates a new [`Error`] from a boxed error.
    pub fn from_boxed(err: Box<dyn core::error::Error + Send + Sync + 'static>) -> Self {
        Self { err }
    }

    pub fn from_err<E>(err: E) -> Self
    where
        E: core::error::Error + Send + Sync + 'static,
    {
        Self { err: Box::new(err) }
    }

    /// Returns the underlying error.
    pub fn into_inner(self) -> Box<dyn core::error::Error + Send + Sync + 'static> {
        self.err
    }
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}
