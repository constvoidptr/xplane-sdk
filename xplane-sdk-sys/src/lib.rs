#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    rustdoc::bare_urls,
    rustdoc::invalid_rust_codeblocks
)]
#![doc = include_str!("../README.md")]

// Generate bindings
#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Use pre-built bindings
#[cfg(not(feature = "generate-bindings"))]
include!("pre-built-bindings.rs");
