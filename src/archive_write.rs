use rarchive_sys::archive;

/// The `WriteArchive` is used to read entries from an archive.
#[derive(Debug, Clone)]
pub struct WriteArchive {
    inner: *mut archive,
}

impl WriteArchive {
    /// Creates a new [`WriteArchive`] object that can be used to read entries from
    /// an archive.
    ///
    /// [`WriteArchive`]: ./struct.WriteArchive.html
    pub fn new() -> Self {
        let inner: *mut archive = unsafe { rarchive_sys::archive_write_new() };
        // This can only fail if the `archive_read_new` method failed
        // to allocate memory.
        assert!(!inner.is_null());
        Self { inner }
    }
}

impl Drop for WriteArchive {
    fn drop(&mut self) {
        unsafe {
            rarchive_sys::archive_write_free(self.inner);
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
