use crate::{Error, Filter, Format, Result};
use rarchive_sys::archive;
use std::{ffi::CString, path::Path};

/// The `ReadArchive` is used to read entries from an archive.
#[derive(Debug, Clone)]
pub struct ReadArchive {
    pub(crate) inner: *mut archive,
}

impl ReadArchive {
    /// Creates a new [`ReadArchive`] object that can be used to read entries from
    /// an archive.
    ///
    /// [`ReadArchive`]: ./struct.ReadArchive.html
    pub fn new() -> Self {
        let inner: *mut archive = unsafe { rarchive_sys::archive_read_new() };
        // This can only fail if the `archive_read_new` method failed
        // to allocate memory.
        assert!(!inner.is_null());
        Self { inner }
    }

    /// Enables the given [`Filter`] for this `ReadArchive`.
    ///
    /// [`Filter`]: ../struct.Filter.html
    pub fn support_filter(&mut self, filter: Filter) {
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

        unsafe { action(self.inner) };
    }

    /// Enables the given [`Format`] for this `ReadArchive`.
    ///
    /// [`Format`]: ../struct.Format.html
    pub fn support_format(&mut self, format: Format) {
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

        unsafe { action(self.inner) };
    }

    /// Opens the given file for this archive.
    ///
    /// This method has to be called before any operation can be done.
    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        const DEFAULT_BLOCKSIZE: u64 = 10240;

        let path = path.as_ref().to_str().expect("path has to be valid utf-8");
        let raw_path = CString::new(path).expect("failed to create path");

        let result = unsafe {
            rarchive_sys::archive_read_open_filename(
                self.inner,
                raw_path.as_ptr(),
                DEFAULT_BLOCKSIZE,
            )
        };

        if result != rarchive_sys::ARCHIVE_OK {
            Err(unsafe { Error::from_read_archive(self) })
        } else {
            Ok(())
        }
    }
}

impl Drop for ReadArchive {
    fn drop(&mut self) {
        unsafe {
            rarchive_sys::archive_read_free(self.inner);
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
