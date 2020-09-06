use crate::{entry::Entry, Archive, Error, Filter, Format, Result};
use rarchive_sys::archive;
use std::{ffi::CString, path::Path, ptr::NonNull};

/// The `ReadArchive` is used to read entries from an archive.
#[derive(Debug, Clone)]
pub struct ReadArchive {
    pub(crate) inner: NonNull<archive>,
}

impl ReadArchive {
    /// Creates a new [`ReadArchive`] object that can be used to read entries from
    /// an archive.
    ///
    /// [`ReadArchive`]: ./struct.ReadArchive.html
    pub fn new() -> Self {
        let inner: *mut archive = unsafe { rarchive_sys::archive_read_new() };
        let inner = NonNull::new(inner).expect("failed to create ReadArchive");
        Self { inner }
    }

    /// Returns an iterator over all entries of this archive.
    pub fn entries(&mut self) -> Entries<'_> {
        Entries { archive: self }
    }
}

impl Archive for ReadArchive {
    fn as_mut_ptr(&self) -> *mut rarchive_sys::archive {
        self.inner.as_ptr()
    }

    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        const DEFAULT_BLOCKSIZE: u64 = 10240;

        let path = path.as_ref().to_str().expect("path has to be valid utf-8");
        let raw_path = CString::new(path).expect("failed to create path");

        let result = unsafe {
            rarchive_sys::archive_read_open_filename(
                self.as_mut_ptr(),
                raw_path.as_ptr(),
                DEFAULT_BLOCKSIZE,
            )
        };

        if result == rarchive_sys::ARCHIVE_OK {
            Ok(())
        } else {
            Err(unsafe { Error::from_archive(self) })
        }
    }

    fn support_format(&mut self, format: Format) {
        let action = match format {
            Format::All => rarchive_sys::archive_read_support_format_all,
            Format::Empty => rarchive_sys::archive_read_support_format_empty,
            Format::SevenZip => rarchive_sys::archive_read_support_format_7zip,
            Format::Ar => rarchive_sys::archive_read_support_format_ar,
            Format::Cab => rarchive_sys::archive_read_support_format_cab,
            Format::Cpio => rarchive_sys::archive_read_support_format_cpio,
            Format::Iso9660 => rarchive_sys::archive_read_support_format_iso9660,
            Format::Lha => rarchive_sys::archive_read_support_format_lha,
            Format::Mtree => rarchive_sys::archive_read_support_format_mtree,
            Format::Rar => rarchive_sys::archive_read_support_format_rar,
            Format::Raw => rarchive_sys::archive_read_support_format_raw,
            Format::Tar => rarchive_sys::archive_read_support_format_tar,
            Format::Xar => rarchive_sys::archive_read_support_format_xar,
            Format::Zip => rarchive_sys::archive_read_support_format_zip,
        };

        unsafe { action(self.as_mut_ptr()) };
    }

    fn support_filter(&mut self, filter: Filter) {
        let action = match filter {
            Filter::All => rarchive_sys::archive_read_support_filter_all,
            Filter::None => rarchive_sys::archive_read_support_filter_none,
            Filter::Bzip2 => rarchive_sys::archive_read_support_filter_bzip2,
            Filter::Compress => rarchive_sys::archive_read_support_filter_compress,
            Filter::Grzip => rarchive_sys::archive_read_support_filter_grzip,
            Filter::Gzip => rarchive_sys::archive_read_support_filter_gzip,
            Filter::Lrzip => rarchive_sys::archive_read_support_filter_lrzip,
            Filter::Lz4 => rarchive_sys::archive_read_support_filter_lz4,
            Filter::Lzma => rarchive_sys::archive_read_support_filter_lzma,
            Filter::Lzop => rarchive_sys::archive_read_support_filter_lzop,
            Filter::Rpm => rarchive_sys::archive_read_support_filter_rpm,
            Filter::Uu => rarchive_sys::archive_read_support_filter_uu,
            Filter::Xz => rarchive_sys::archive_read_support_filter_xz,
            Filter::Zstd => rarchive_sys::archive_read_support_filter_zstd,
        };

        unsafe { action(self.as_mut_ptr()) };
    }
}

impl Drop for ReadArchive {
    fn drop(&mut self) {
        unsafe {
            rarchive_sys::archive_read_free(self.as_mut_ptr());
        }
    }
}

/// An iterator over the entries of an archive.
pub struct Entries<'archive> {
    archive: &'archive ReadArchive,
}

impl<'archive> Iterator for Entries<'archive> {
    type Item = Entry<'archive>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = Entry::new();
        let entry_ptr = entry.as_mut_ptr();

        let result = unsafe {
            rarchive_sys::archive_read_next_header2(self.archive.as_mut_ptr(), entry_ptr)
        };
        if result == rarchive_sys::ARCHIVE_OK {
            Some(entry)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _a1 = ReadArchive::new();
    }

    #[test]
    fn test_support() {
        let mut a = ReadArchive::new();
        a.support_format(Format::All);
        a.support_filter(Filter::All);
    }
}
