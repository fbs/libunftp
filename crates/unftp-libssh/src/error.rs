// mostly from https://github.com/wez/libssh-rs/blob/main/libssh-rs/src/error.rs
//
//
//
use std::ffi::{c_void, CStr};

use libssh_rs_sys as sys;
use thiserror::Error;

/// Represents an error condition
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// The last request was denied but situation is recoverable
    #[error("RequestDenied: {}", .0)]
    RequestDenied(String),
    /// A fatal error occurred. This could be an unexpected disconnection
    #[error("Fatal: {}", .0)]
    Fatal(String),
    /// The session is in non-blocking mode and the call must be tried again
    #[error("TryAgain")]
    TryAgain,
}

/// Represents the result of a fallible operation
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn is_try_again(&self) -> bool {
        matches!(self, Self::TryAgain)
    }

    pub fn fatal<S: Into<String>>(s: S) -> Self {
        Self::Fatal(s.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::fatal(err.to_string())
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        match err {
            Error::TryAgain => std::io::Error::new(std::io::ErrorKind::WouldBlock, "TryAgain"),
            Error::RequestDenied(msg) | Error::Fatal(msg) => std::io::Error::new(std::io::ErrorKind::Other, msg),
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Error {
        Error::Fatal(err.to_string())
    }
}

pub(crate) fn is_error(rc: std::os::raw::c_int) -> bool {
    !(rc == (sys::SSH_OK as i32))
}

pub(crate) fn get_error(e: *mut c_void) -> Option<Error> {
    let code = unsafe { sys::ssh_get_error_code(e) } as sys::ssh_error_types_e;

    if code == sys::ssh_error_types_e_SSH_NO_ERROR {
        return None;
    }

    let msg = unsafe { sys::ssh_get_error(e) };
    let msg = if msg.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(msg) }.to_string_lossy().to_string()
    };

    if code == sys::ssh_error_types_e_SSH_REQUEST_DENIED {
        Some(Error::RequestDenied(msg))
    } else if code == sys::ssh_error_types_e_SSH_EINTR {
        Some(Error::TryAgain)
    } else {
        Some(Error::Fatal(msg))
    }
}
