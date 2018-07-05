use failure::Fail;
use std::error::Error;
use std::fmt::{self, Debug, Display};

crate struct StdError<E: Error + Send + Sync + 'static> {
    crate inner: E,
}

impl<E: Error + Send + Sync + 'static> Display for StdError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<E: Error + Send + Sync + 'static> Debug for StdError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<E: Error + Send + Sync + 'static> Fail for StdError<E> {}

crate trait IntoError {
    fn into_error(self) -> failure::Error;
}

impl<E: Error + Send + Sync + 'static> IntoError for E {
    fn into_error(self) -> failure::Error {
        failure::Error::from(StdError { inner: self })
    }
}
