use bindings::ffi;
use std::ffi::CStr;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    tcod_error: ffi::TCOD_Error,
    tcod_error_message: String,
}

impl Error {
    #[inline]
    pub fn is_warning(tcod_error: ffi::TCOD_Error) -> bool {
        matches!(tcod_error, ffi::TCOD_Error::TCOD_E_WARN)
    }

    #[inline]
    pub fn is_ok(tcod_error: ffi::TCOD_Error) -> bool {
        matches!(tcod_error, ffi::TCOD_Error::TCOD_E_OK)
    }

    #[inline]
    pub fn is_error(tcod_error: ffi::TCOD_Error) -> bool {
        !Self::is_warning(tcod_error) && !Self::is_ok(tcod_error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tcod_error_message)
    }
}

impl std::error::Error for Error {}

impl From<ffi::TCOD_Error> for Error {
    fn from(e: ffi::TCOD_Error) -> Self {
        assert!(Self::is_error(e));
        let error_message = unsafe { ffi::TCOD_get_error() };
        assert!(!error_message.is_null());
        let error_message = unsafe { CStr::from_ptr(error_message) };

        Self {
            tcod_error: e,
            tcod_error_message: error_message.to_string_lossy().into_owned(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
