use crate::ReadArchive;
use std::{ffi::CStr, fmt};

/// The global result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The central error type that is used for all operations that can fail.
#[derive(Debug)]
pub struct Error {
    msg: Option<String>,
    errno: i32,
}

impl Error {
    /// Creates an `Error` by calling `archive_errno` and `archive_error_string`
    /// functions to retrieve error information.
    pub(crate) unsafe fn from_read_archive(archive: &ReadArchive) -> Self {
        let code = rarchive_sys::archive_errno(archive.inner);
        let raw_msg = rarchive_sys::archive_error_string(archive.inner);
        let raw_msg = if raw_msg.is_null() {
            None
        } else {
            Some(CStr::from_ptr(raw_msg))
        };

        Self {
            errno: code,
            msg: raw_msg.map(|msg| msg.to_string_lossy().into_owned()),
        }
    }

    /// Returns the error code that was returned by libarchive.
    pub fn code(&self) -> i32 {
        self.errno
    }

    /// Returns the message of this error.
    pub fn message(&self) -> Option<&str> {
        self.msg.as_deref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = self.message() {
            write!(f, "error {}: {}", self.code(), msg)
        } else {
            write!(f, "error {}", self.code())
        }
    }
}
