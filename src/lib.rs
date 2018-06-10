//! Rust binding for librpm: the RPM Package Manager library
//!
//! This crate contains idiomatic Rust bindings which aim to expose a safe
//! API to librpm. The low-level unsafe bindings are located in the
//! [librpm-sys] crate, which is automatically generated by bindgen.
//!
//! Make sure to call `librpm::read_config` to load rpmrc configuration.
//!
//! See the `librpm::db::Database` type for examples of how to interact with
//! the RPM database.
//!
//! [librpm-sys]: https://rustrpm.org/librpm_sys/index.html

#![crate_name = "librpm"]
#![crate_type = "rlib"]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![doc(html_root_url = "https://rustrpm.org/librpm/")]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate librpm_sys;
extern crate streaming_iterator;

/// Error types (defined first due to macros)
#[macro_use]
pub mod error;

/// RPM configuration (i.e. rpmrc)
pub mod config;

/// RPM database access
pub mod db;

/// Internal functionality not to be exposed outside of this crate
mod internal;

/// Package licenses
pub mod license;

/// Macros are RPM's configuration system
pub mod macro_context;

/// RPM packages
pub mod package;

/// Package versions
pub mod version;

pub use db::Index;
pub use error::Error;
pub use license::License;
pub use macro_context::MacroContext;
pub use package::Package;
pub use version::Version;