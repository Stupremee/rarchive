use crate::Archive;
use std::{ffi::CStr, fmt};

/// The global result type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error that can be either a normal libarchive [`Error`], or an
/// [`Rust I/O`] error.
///
/// [`Error`]: ./struct.Error.html
/// [`Rust I/O`]: https://doc.rust-lang.org/std/io/struct.Error.html
#[derive(Debug, thiserror::Error)]
pub enum ErrorOrIo {
    #[error("archive error: {0}")]
    Libarchive(#[from] Error),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

/// The central error type that is used for all operations that can fail.
#[derive(Debug)]
pub struct Error {
    msg: Option<String>,
    errno: i32,
}

impl Error {
    /// Calls the given function and checks the returned code if it's an error.
    pub(crate) fn from_code(
        archive: &dyn Archive,
        func: impl FnOnce() -> std::os::raw::c_int,
    ) -> Result<()> {
        let result = func();
        if result != rarchive_sys::ARCHIVE_OK && result != rarchive_sys::ARCHIVE_WARN {
            Err(unsafe { Self::from_archive(archive) })
        } else {
            Ok(())
        }
    }

    /// Creates an `Error` by calling `archive_errno` and `archive_error_string`
    /// functions to retrieve error information.
    pub(crate) unsafe fn from_archive(archive: &dyn Archive) -> Self {
        let code = rarchive_sys::archive_errno(archive.as_mut_ptr());
        let raw_msg = rarchive_sys::archive_error_string(archive.as_mut_ptr());
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

impl std::error::Error for Error {}
