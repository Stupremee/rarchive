//! This library provides raw FFI bindings to [libarchive].
//!
//! Documentation of the libarchive can be found [here](https://github.com/libarchive/libarchive/wiki).
//!
//! Following the `-sys` crate convention this crate only
//! exports the [libarchive] FFI bindings and doesn't define
//! any new types or high level bindings.
//!
//! [libarchive]: https://github.com/libarchive/libarchive
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const ARCHIVE_EOF: i32 = 1;
pub const ARCHIVE_OK: i32 = 0;
pub const ARCHIVE_RETRY: i32 = -10;
pub const ARCHIVE_WARN: i32 = -20;
pub const ARCHIVE_FAILED: i32 = -25;
pub const ARCHIVE_FATAL: i32 = -30;
