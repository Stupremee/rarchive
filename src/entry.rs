use rarchive_sys::archive_entry;

/// Represents an entry in an archive file.
///
/// An `Entry` contains a `struct stat` (from the linux kernel)
/// plus a pathname, textual group, user names, etc.
/// It's used to represent any metadat associated with an entry in an archive.
#[derive(Debug)]
pub struct Entry {
    inner: *mut archive_entry,
}

impl Entry {
    /// Allocate and return a blank struct archive_entry object.
    pub fn new() -> Self {
        let inner = unsafe { rarchive_sys::archive_entry_new() };
        assert!(!inner.is_null());
        Self { inner }
    }

    /// Returns the underlying pointer to the raw libarchive
    /// `archive_entry` struct.
    pub fn as_ptr(&self) -> *const archive_entry {
        self.inner
    }

    /// Returns the underlying mutable pointer to the raw libarchive
    /// `archive_entry` struct.
    pub fn as_mut_ptr(&self) -> *mut archive_entry {
        self.inner
    }
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        let new_entry = unsafe { rarchive_sys::archive_entry_clone(self.inner) };
        assert!(!new_entry.is_null());
        Self { inner: new_entry }
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        unsafe { rarchive_sys::archive_entry_free(self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let _e = Entry::new();
    }

    #[test]
    fn test_clone_entry() {
        let e1 = Entry::new();
        let _e2 = e1.clone();
    }
}
