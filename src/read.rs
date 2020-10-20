use crate::{Archive, Error, ErrorOrIo, Filter, Format, Result};
use rarchive_sys::archive;
use std::{path::Path, ptr::NonNull};

/// `ReadArchive` is used to read an archive.
pub struct ReadArchive {
    inner: NonNull<archive>,
}

impl ReadArchive {
    /// Creates a new archive, add all filters and formats, and then tries
    /// to open the file.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ErrorOrIo> {
        let mut archive = Self::new();
        let _ = archive.support_format(Format::All);
        let _ = archive.support_filter(Filter::All);
        archive.open(path)?;
        Ok(archive)
    }

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

    /// Tries to open an archive by reading the data from a file.
    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ErrorOrIo> {
        let buf = std::fs::read(path)?;
        self.open_buffer(buf.as_slice())?;
        Ok(())
    }

    /// Tries to open an archive by reading the data from a raw byte buffer.
    pub fn open_buffer<Buf: AsRef<[u8]>>(&mut self, buf: Buf) -> Result<()> {
        let buf = buf.as_ref();

        Error::from_code(self, || unsafe {
            rarchive_sys::archive_read_open_memory(
                self.as_mut_ptr(),
                buf.as_ptr() as _,
                buf.len() as u64,
            )
        })
    }
}

impl Archive for ReadArchive {
    fn as_mut_ptr(&self) -> *mut rarchive_sys::archive {
        self.inner.as_ptr()
    }

    fn support_format(&mut self, format: Format) -> Result<()> {
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
            Format::Tar => rarchive_sys::archive_read_support_format_tar,
            Format::Xar => rarchive_sys::archive_read_support_format_xar,
            Format::Zip => rarchive_sys::archive_read_support_format_zip,
        };

        Error::from_code(self, || unsafe { action(self.as_mut_ptr()) })
    }

    fn support_filter(&mut self, filter: Filter) -> Result<()> {
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

        Error::from_code(self, || unsafe { action(self.as_mut_ptr()) })
    }

    fn set_filter_option(&mut self, module: &str, option: &str, value: &str) -> Result<()> {
        Error::from_code(self, || unsafe {
            rarchive_sys::archive_read_set_filter_option(
                self.as_mut_ptr(),
                module.as_ptr() as _,
                option.as_ptr() as _,
                value.as_ptr() as _,
            )
        })
    }

    fn set_format_option(&mut self, module: &str, option: &str, value: &str) -> Result<()> {
        Error::from_code(self, || unsafe {
            rarchive_sys::archive_read_set_format_option(
                self.as_mut_ptr(),
                module.as_ptr() as _,
                option.as_ptr() as _,
                value.as_ptr() as _,
            )
        })
    }

    fn set_option(&mut self, module: &str, option: &str, value: &str) -> Result<()> {
        Error::from_code(self, || unsafe {
            rarchive_sys::archive_read_set_option(
                self.as_mut_ptr(),
                module.as_ptr() as _,
                option.as_ptr() as _,
                value.as_ptr() as _,
            )
        })
    }

    fn set_options(&mut self, options: &str) -> Result<()> {
        Error::from_code(self, || unsafe {
            rarchive_sys::archive_read_set_options(self.as_mut_ptr(), options.as_ptr() as _)
        })
    }
}

impl Drop for ReadArchive {
    fn drop(&mut self) {
        unsafe {
            rarchive_sys::archive_read_free(self.as_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_archive() {
        let _a1 = ReadArchive::new();
    }

    #[test]
    fn support_filter_format() {
        let mut a = ReadArchive::new();
        a.support_filter(Filter::All).unwrap();
        a.support_format(Format::All).unwrap();
    }

    #[test]
    fn set_option() {
        let mut a = ReadArchive::new();
        assert!(a.set_options("invalid").is_err());
    }

    #[test]
    fn open_buf() {
        let mut a = ReadArchive::new();
        a.support_format(Format::All).unwrap();
        assert!(a.open_buffer([0u8]).is_err());
    }

    #[test]
    fn open_path() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/empty.zip");
        let _a = ReadArchive::from_path(path).unwrap();
    }
}
