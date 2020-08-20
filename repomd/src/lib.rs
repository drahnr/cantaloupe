mod error;
pub mod xml;
pub use xml::*;
pub mod servings;
pub use servings::*;

pub use error::{Error, Result};

use std::path::{PathBuf,Path};

use url::Url;
use std::collections::HashSet;
#[derive(Debug)]
pub struct Repo {
    url : Url,
    packages : HashSet<Package>,
}

impl Repo {
    pub fn new(url : Url) -> Self {
        Self {
            url : url,
            packages : HashSet::with_capacity(32)
        }
    }

    pub fn packages(&self) -> impl Iterator<Item=&Package> {
        self.packages.iter()
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
}


#[derive(Debug)]
pub struct Package {
    files : HashSet<PathBuf>,
}

impl Package {
    pub fn files(&self) -> impl Iterator<Item=&Path> {
        self.files.iter().map(|pb| { pb.as_ref()})
    }

    pub fn hash<'x,'y>(&'y self) -> &'x [u8] {
        &[]
    }

    pub fn open_hash<'x,'y>(&'y self) -> &'x [u8] {
        &[]
    }

    pub fn identifier(&self) -> String {
        "124".to_string()
    }

    pub fn arch(&self) -> String {
        "x86_64".to_string()
    }

    pub fn version(&self) -> String {
        "".to_string()
    }

    pub fn rel(&self) -> String {
        "88".to_string()
    }

    pub fn epoch(&self) -> u64 {
        1u64
    }

    pub fn name(&self) -> String {
        "".to_string()
    }
}


#[cfg(test)]
mod integration;