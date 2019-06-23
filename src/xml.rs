use crate::errors::*;
use crate::repo::Repo;
use sha2::{Digest,Sha256};
use bytes::Bytes;

use common_failures::prelude::*;

use std::marker::PhantomData;

use compression::prelude::*;

use crate::failure::Fail;

pub trait XmlRender {
    fn xml_render(&self) -> Result<String>;


    fn compressed_xml_bytes(&self) -> Result<Bytes> {
        let s = self.xml_render()?;
        let bytes = s.as_bytes()
            .into_iter()
            .cloned()
            .encode(&mut GZipEncoder::new(), Action::Finish)
            .collect::<std::result::Result<Vec<_>, _>>().map_err(|err| {
                CantaError::CompressionFailed.context(err)
            })?;
        Ok(Bytes::from(bytes))
    }

    fn xml_digest(&self) -> Result<String> {
        self.xml_render().and_then(|s| {
            let mut digest = Sha256::new();
            digest.input(s.as_bytes());
            let digest = digest.result();
            let digest_str = format!("{:x}", digest);
            Ok(digest_str)
        })
    }

    fn compressed_xml_digest(&self) -> Result<String> {
        self.compressed_xml_bytes().and_then(|bytes| {
            let mut digest = Sha256::new();
            digest.input(bytes);
            let digest = digest.result();

            let digest_str = format!("{:x}", digest);
            Ok(digest_str)
        })
    }


    fn digest_name(&self) -> String {
        "sha256".to_string()
    }

    fn compression_name() -> String {
       "gz".to_string() 
    }
}



#[derive(Debug)]
pub struct Index<'a> {
    repo: &'a Repo
}

impl<'a> XmlRender for Index<'a> {
    fn xml_render(&self) -> Result<String> {
        Ok(
r#"I am the index :)"#.to_string()
        )
    }
}

impl<'a> Index<'a> {
    pub fn new(repo : &'a Repo) -> Self {
        Self { repo }
    }
}




#[derive(Debug)]
pub struct Other<'a> {
    phantomas: PhantomData<&'a str>
}

impl<'a> XmlRender for Other<'a> {
    fn xml_render(&self) -> Result<String> {
        Ok(
r#"
<?xml version="1.0" encoding="UTF-8"?>
<otherdata xmlns="http://linux.duke.edu/metadata/other" packages="{{ packages.count }}">
</otherdata>"#.to_string()
        )
    }
}


#[derive(Debug)]
pub struct Primary<'a> {
    repo : &'a Repo,
}

impl<'a> Primary<'a> {
    pub fn new(repo : &'a Repo) -> Self {
        Self {repo}
    }
}

impl<'a> XmlRender for Primary<'a> {
    fn xml_render(&self) -> Result<String> {
        let package_count = self.repo.count_packages();
        Ok(format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<metadata xmlns="http://linux.duke.edu/metadata/common" xmlns:rpm="http://linux.duke.edu/metadata/rpm" packages="{package_count}">
</metadata>"#, package_count=package_count)
        )
    }
}


#[derive(Debug)]
pub struct FileList<'a> {
    repo: &'a Repo,
}

impl<'a> FileList<'a> {
    pub fn new(repo : &'a Repo) -> Self {
        Self {repo}
    }

    pub fn timestamp(&self) -> u64 {
        self.repo.last_update()
    }
}

impl<'a> XmlRender for FileList<'a> {
    fn xml_render(&self) -> Result<String> {
        let mut s = String::with_capacity(500);
        let package_count = self.repo.count_packages();

        let hdr_s = format!(
r#"
<?xml version="1.0" encoding="UTF-8"?>
<filelists xmlns="http://linux.duke.edu/metadata/filelists" packages="{package_count}">
"#, package_count = package_count);
        s.push_str(hdr_s.as_str());

        for pkg in self.repo.packages() {
            let pkg_s = format!(
r#"
  <package pkgid="{identifier}" name="{name}" arch="{arch}">
  <version epoch="{epoch}" ver="{version}" rel="{rel}" />
"#,
                identifier=pkg.identifier(), name=pkg.name(), arch=pkg.arch(),
                epoch=pkg.epoch(), version=pkg.version(), rel=pkg.rel());
            s.push_str(pkg_s.as_str());
            for file in pkg.files() {
                let file_s = format!(
r#"    <file>{path}</file>"#, path=file.display()
                );
                s.push_str(file_s.as_str());
            }
        }

        s.push_str(r#"</filelists>"#);
        Ok(s)
    }
}


#[derive(Debug)]
pub struct RepoMetaData<'a> {
    repo: &'a Repo,
}

impl<'a> RepoMetaData<'a> {
    pub fn new(repo : &'a Repo) -> Self {
        Self {repo}
    }

    pub fn timestamp(&self) -> u64 {
        self.repo.last_update()
    }
}

impl<'a> XmlRender for RepoMetaData<'a> {
    fn xml_render(&self) -> Result<String> {
        let primary = Primary::new(self.repo);
        let filelist = FileList::new(self.repo);

        Ok(format!(
r#"
<?xml version="1.0" encoding="UTF-8"?>
<repomd xmlns="{url}">
<data type="primary">
    <location href="repodata/primary.xml.gz"/>
    <checksum type="{digest_name}">{primary_digest}</checksum>
    <timestamp>{timestamp}</timestamp>
    <open-checksum type="{digest_name}">{primary_uncompressed_digest}</open-checksum>
</data>
<data type="filelists">
    <location href="repodata/filelists.xml.gz"/>
    <checksum type="{digest_name}">{filelist_digest}</checksum>
    <timestamp>{timestamp}</timestamp>
    <open-checksum type="{digest_name}">{filelist_uncompressed_digest}</open-checksum>
</data>
"#,
    url = self.repo.url().as_str(),
    timestamp = self.timestamp(),
    digest_name = primary.digest_name(),
    primary_digest = primary.compressed_xml_digest()?,
    primary_uncompressed_digest = primary.xml_digest()?,
    filelist_digest = filelist.compressed_xml_digest()?,
    filelist_uncompressed_digest = filelist.xml_digest()?,

))
    }
}
    

