use console::style;
use failure;
use std::fmt;

#[derive(Debug)]
pub struct InternalThorError {
    status: i32,
    context: String,
    error: failure::Error,
}

#[derive(Debug)]
pub struct ExternalThorError {
    error: InternalThorError,
    message: String,
}

#[derive(Debug)]
pub enum ThorError {
    Internal(InternalThorError),
    External(ExternalThorError),
}

impl ThorError {
    pub fn format(&'error self, verbose: bool) -> ThorErrorFormatter<'error> {
        ThorErrorFormatter::new(self, verbose)
    }
}

impl From<failure::Context<String>> for ThorError {
    fn from(error: failure::Context<String>) -> ThorError {
        ThorError::Internal(InternalThorError {
            status: 1,
            context: error.get_context().clone(),
            error: error.into(),
        })
    }
}

#[derive(new)]
pub struct ThorErrorFormatter<'error> {
    error: &'error ThorError,
    verbose: bool,
}

impl fmt::Display for ThorErrorFormatter<'error> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error {
            ThorError::Internal(e) => {
                writeln!(f, "{}", style("An unexpected error occurred").red())?;

                if self.verbose {
                    writeln!(f, "{}\n", e.context)?;
                    writeln!(f, "{:?}", e.error)?;
                }

                Ok(())
            }

            ThorError::External(e) => writeln!(f, "{:?}", e.message),
        }
    }
}

pub trait WithUserFriendlyContext: ErrorCode {
    fn with_user_friendly_context(self, context: String) -> ThorError;
}

pub trait ErrorCode {
    fn error_code(&self) -> i32;
}

impl<E: std::error::Error> ErrorCode for E {
    fn error_code(&self) -> i32 {
        1
    }
}

impl<E: std::error::Error + ErrorCode + Sync + Send + 'static> WithUserFriendlyContext for E {
    fn with_user_friendly_context(self, context: String) -> ThorError {
        ThorError::Internal(InternalThorError {
            status: self.error_code(),
            error: self.into(),
            context,
        })
    }
}

pub trait ResultWithUserFriendlyContext {
    type Ok;
    type Error;

    fn with_user_friendly_context<F: FnOnce(&Self::Error) -> String>(
        self,
        context: F,
    ) -> Result<Self::Ok, ThorError>;
}

impl<T, E: std::error::Error + ErrorCode + Sync + Send + 'static> ResultWithUserFriendlyContext
    for Result<T, E>
{
    type Ok = T;
    type Error = E;

    fn with_user_friendly_context<F: FnOnce(&Self::Error) -> String>(
        self,
        context: F,
    ) -> Result<Self::Ok, ThorError> {
        self.map_err(|e| {
            let ctx = context(&e);
            e.with_user_friendly_context(ctx)
        })
    }
}
