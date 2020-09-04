#![deny(rust_2018_idioms)]
// TODO: Uncomment this
// #![deny(missing_docs)]
#![warn(clippy::pedantic)]

mod archive_read;
pub use archive_read::*;

mod archive_write;
pub use archive_write::*;

/// Representing every possible filter that is available to read / write
/// from / to.
///
/// **Note:** Some filters, e.g. zstd or lz4, will fall back to their
/// according command line program (`zstd`, or `lz4`)
#[derive(Debug, Clone, Copy)]
pub enum Filter {
    /// Enables all available filters.
    All,
    /// Enable no filter.
    None,
    Bzip2,
    Compress,
    Grzip,
    Gzip,
    Lrzip,
    Lz4,
    Lzma,
    Lzop,
    Rpm,
    Uu,
    Xz,
    Zstd,
    // TODO: Probably support `program` filter.
}

/// Representing every possible format that is available to read / write
/// from / to.
pub enum Format {
    /// Enable all available formats.
    All,
    /// Enable the `empty` format, which is just a format,
    /// that is able to read empty files.
    ///
    /// **Warning:** This is not the same as [`Filter::None`].
    ///
    /// [`Filter::None`]: ./enum.Filter.html#None
    Empty,
    /// The 7zip format.
    SevenZip,
    Ar,
    Cab,
    Cpio,
    Iso9660,
    Lha,
    Mtree,
    Rar,
    Raw,
    Tar,
    Xar,
    Zip,
}
