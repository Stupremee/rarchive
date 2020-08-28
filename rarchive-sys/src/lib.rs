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
