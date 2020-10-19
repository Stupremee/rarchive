use rarchive_sys::archive;
use std::ptr::NonNull;

/// `ReadArchive` is used to read an archive.
pub struct ReadArchive {
    inner: NonNull<archive>,
}

impl ReadArchive {
    /// Builds a new `ReadArchive` object.
    ///
    /// # Panics
    ///
    /// This method will panic if it fails to allocate memory for
    /// the `ReadArchive` object.
    pub fn new() -> Self {
        let inner = unsafe { rarchive_sys::archive_read_new() };
        let inner = NonNull::new(inner).expect("failed to create ReadArchive object");
        Self { inner }
    }
}
