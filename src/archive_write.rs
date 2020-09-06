use rarchive_sys::archive;
use std::ptr::NonNull;

/// The `WriteArchive` is used to read entries from an archive.
#[derive(Debug, Clone)]
pub struct WriteArchive {
    inner: NonNull<archive>,
}

impl WriteArchive {
    /// Creates a new [`WriteArchive`] object that can be used to read entries from
    /// an archive.
    ///
    /// [`WriteArchive`]: ./struct.WriteArchive.html
    pub fn new() -> Self {
        let inner: *mut archive = unsafe { rarchive_sys::archive_write_new() };
        let inner = NonNull::new(inner).expect("failed to create WriteArchive");
        Self { inner }
    }
}

impl Drop for WriteArchive {
    fn drop(&mut self) {
        unsafe {
            rarchive_sys::archive_write_free(self.inner.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _a1 = WriteArchive::new();
    }
}
