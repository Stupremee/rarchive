use rarchive_sys::archive_entry;
use std::{ffi::CStr, marker::PhantomData};

/// Represents an entry in an archive file.
///
/// An `Entry` contains a `struct stat` (from the linux kernel)
/// plus a pathname, textual group, user names, etc.
/// It's used to represent any metadat associated with an entry in an archive.
#[derive(Debug)]
pub struct Entry<'archive> {
    inner: *mut archive_entry,
    _lifetime: PhantomData<&'archive ()>,
}

impl Entry<'_> {
    /// Allocate and return a blank struct archive_entry object.
    pub fn new() -> Self {
        let inner = unsafe { rarchive_sys::archive_entry_new() };
        assert!(!inner.is_null());
        Self {
            inner,
            _lifetime: PhantomData,
        }
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

    /// Returns the pathname of this entry.
    ///
    /// This method will retrieve the pathname from libarchive,
    /// and then allocate it again.
    pub fn pathname(&self) -> String {
        let raw_name = unsafe { rarchive_sys::archive_entry_pathname(self.inner) };
        assert!(!raw_name.is_null());

        let name = unsafe { CStr::from_ptr(raw_name) }
            .to_str()
            .expect("the raw_name should always be utf-8");
        name.into()
    }
}

impl Clone for Entry<'_> {
    fn clone(&self) -> Self {
        let new_entry = unsafe { rarchive_sys::archive_entry_clone(self.inner) };
        assert!(!new_entry.is_null());
        Self {
            inner: new_entry,
            _lifetime: PhantomData,
        }
    }
}

impl Drop for Entry<'_> {
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
