mod error;
pub mod xml;
pub use xml::*;
pub mod servings;
pub use servings::*;

pub use error::{Error, Result};

use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::collections::HashMap;

use url::Url;

#[derive(Debug)]
pub struct Repo {
    url: Url,
    arch: Arch,
    packages: HashMap<String, HashMap<Version, Package>>,
}

impl Repo {
    pub fn new<U: Into<Url>>(url: U) -> Self {
        Self {
            url: url.into(),
            arch: Arch::default(),
            packages: HashMap::with_capacity(128),
        }
    }

    pub fn packages<'a>(&'a self) -> impl Iterator<Item = (String, Version, Package)> + 'a {
        self.packages
            .iter()
            .map(|(name, versions)| {
                let name = name.to_owned();
                versions
                    .iter()
                    .map(move |(version, package): (&Version, &Package)| {
                        let package: Package = (*package).clone();
                        (name.clone(), version.clone(), package)
                    })
            })
            .flatten()
    }

    pub fn count_packages(&self) -> usize {
        self.packages.len()
    }

    /// timestamp of last update
    pub fn last_update(&self) -> u64 {
        0u64
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    /// Add a package to the repository
    pub fn add_package(&mut self, pkg: Package) {
        let _ = self
            .packages
            .entry(pkg.name().to_owned())
            .or_default()
            .insert(pkg.version().clone(), pkg);
    }
}

#[allow(non_camel_case_types)]
#[derive(
    Hash, PartialEq, Eq, Debug, Clone, Copy, strum_macros::Display, strum_macros::EnumString,
)]
pub enum Arch {
    x86_64,
    i386,
    i486,
    i586,
    i686,
}

impl Default for Arch {
    fn default() -> Self {
        Self::x86_64
    }
}

/// Not guaranteed to be a semversion, so we must handle arbitrary strings here.
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Version(String);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}
impl FromStr for  Version {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // @todo may not contain dashes
        Ok(Self(s.to_owned()))
    }
}


#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Release(String);

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for  Release {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

use std::hash::Hash;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Package {
    pub(crate) name: String,
    pub(crate) epoch: u64,
    pub(crate) release: Release,
    pub(crate) version: Version,
    pub(crate) arch: Arch,
    pub(crate) files: Vec<PathBuf>,
    pub(crate) identifier: String,
}

impl Package {
    pub fn files(&self) -> impl Iterator<Item = &Path> {
        self.files.iter().map(|pb| pb.as_ref())
    }

    pub fn hash<'x, 'y>(&'y self) -> &'x [u8] {
        &[]
    }

    pub fn open_hash<'x, 'y>(&'y self) -> &'x [u8] {
        &[]
    }

    /// A `sha256` of the full rpm file content (lead, headers, cpio).
    pub fn identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn arch(&self) -> Arch {
        self.arch
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn release(&self) -> &Release {
        &self.release
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod integration;
