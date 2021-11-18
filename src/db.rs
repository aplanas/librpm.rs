//! RPM database access
//!
//! The database used is whichever one is configured as the `_dbpath` in the
//! in the global macro context. By default this is unset: you will need to
//! call `librpm::config::read_file(None)` to read the default "rpmrc"
//! configuration.
//!
//! # Example
//!
//! Finding the "rpm-devel" RPM in the database:
//!
//! ```
//! use librpm::Index;
//!
//! librpm::config::read_file(None).unwrap();
//!
//! let mut matches = Index::Name.find("rpm-devel");
//! let package = matches.next().unwrap();
//!
//! println!("package name: {}", package.name());
//! println!("package summary: {}", package.summary());
//! println!("package version: {}", package.version());
//! ```

use crate::internal::{iterator::MatchIterator, tag::Tag};
use crate::package::Package;
use streaming_iterator::StreamingIterator;

/// Iterator over the RPM database which returns `Package` structs.
pub struct Iter(MatchIterator);

impl Iterator for Iter {
    type Item = Package;

    /// Obtain the next header from the iterator.
    fn next(&mut self) -> Option<Package> {
        self.0.next().map(|h| h.to_package())
    }
}

/// Searchable fields in the RPM package headers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Index {
    /// Search by package name.
    Name,

    /// Search by package version.
    Version,

    /// Search by package license.
    License,

    /// Search by package summary.
    Summary,

    /// Search by package description.
    Description,

    /// Search by provides name.
    ProvideName,

    /// Search by installed filename.
    InstFilenames,
}

impl Index {
    /// Find an exact match in the given index
    pub fn find<S: AsRef<str>>(self, key: S) -> Iter {
        Iter(MatchIterator::new(self.into(), Some(key.as_ref())))
    }
}

/// Find all packages installed on the local system.
pub fn installed_packages() -> Iter {
    Iter(MatchIterator::new(Tag::NAME, None))
}

/// Find installed packages with a search key that exactly matches the given tag.
///
/// Panics if the glob contains null bytes.
pub fn find<S: AsRef<str>>(index: Index, key: S) -> Iter {
    index.find(key)
}
