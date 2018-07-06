use crate::prelude::*;

crate struct StdError<E: std::error::Error + Send + Sync + 'static> {
    crate inner: E,
}

impl<E: std::error::Error + Send + Sync + 'static> Display for StdError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<E: std::error::Error + Send + Sync + 'static> Debug for StdError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<E: std::error::Error + Send + Sync + 'static> Fail for StdError<E> {}

crate trait IntoError {
    fn into_error(self) -> ArgonError;
}

impl<E: std::error::Error + Send + Sync + 'static> IntoError for E {
    fn into_error(self) -> ArgonError {
        let error = failure::Error::from(StdError { inner: self });
        ArgonError::Error(error)
    }
}
