/*
 * Copyright (C) RustRPM Developers
 *
 * Licensed under the Mozilla Public License Version 2.0
 * Fedora-License-Identifier: MPLv2.0
 * SPDX-2.0-License-Identifier: MPL-2.0
 * SPDX-3.0-License-Identifier: MPL-2.0
 *
 * This is free software.
 * For more information on the license, see LICENSE.
 * For more information on free software, see <https://www.gnu.org/philosophy/free-sw.en.html>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at <https://mozilla.org/MPL/2.0/>.
 */


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

#![doc(html_root_url = "https://rustrpm.org/librpm/")]
#![warn(missing_docs, trivial_casts, unused_qualifications)]
#![warn(rust_2018_idioms)]

/// Error types (defined first due to macros)
#[macro_use]
pub mod error;

/// RPM configuration (i.e. rpmrc)
pub mod config;

/// RPM database access
pub mod db;

/// Internal functionality not to be exposed outside of this crate
mod internal;

/// Macros are RPM's configuration system
pub mod macro_context;

/// RPM packages
pub mod package;

pub use self::{db::Index, error::Error, macro_context::MacroContext, package::Package};
