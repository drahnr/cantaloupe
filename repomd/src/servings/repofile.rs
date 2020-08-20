use super::*;

use serde::Serialize;
use serde::Deserialize;
use toml;
use indexmap::IndexMap;
use chrono::Duration;
use strum;
use strum_macros;

#[derive(Serialize, Deserialize, Clone, Copy, strum_macros::EnumString)]
enum RepoFormat {
    #[strum(serialize="rpm-md",serialize="rpm")]
    RepoMD,
}

impl Default for RepoFormat {
    fn default() -> Self {
        Self::RepoMD
    }
}


/// One block of a repository file
#[derive(Serialize, Deserialize, Clone, Default)]
struct RepoFileBlock {
    name: String,
    baseurl: Option<Url>,
    metalink: Option<Url>,
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    protect: bool,
    #[serde(default)]
    gpgcheck: bool,
    metadata_expire: u64, // @todo use a custom type based on 7y 8m 7d 1h 5s notation
    #[serde(default)]
    autorefresh: bool,
    #[serde(default,alias="type")]
    format: RepoFormat,
}


/// A repository description for a client such as `dnf` or `zypper`
#[derive(Serialize, Deserialize, Clone)]
struct RepoFile {
    pub multi: IndexMap<String, RepoFileBlock>,
}

// @todo custom serialization implementation


// #[derive(Default)]
// struct RepoDescBuilder {
//     shortname: Option<String>,
//     name: String,
//     baseurl: Option<Url>,
//     enabled: bool,
//     protect: bool,
//     gpgcheck: bool,
//     metadata_expire: Duration,
//     autorefresh: bool,
// }

// impl RepoDescBuilder {
//     fn shortname(self, shortname: impl ToString) -> Self {

//     }

//     fn url(url: Url) -> {
//         self 
//     }

//     fn build() -> RepoDesc {
//         RepoDesc {
//             shortname: self.shortname.unwrap(),
//             shortname: self.shortname.unwrap(),
//         }
//     }
// }


impl RepoFile {
    // fn builder() -> RepoDescBuilder {
    //     RepoDescBuilder::default()
    // }
    fn as_toml(&self) -> Result<String> {
        Ok(toml::to_string(self)?)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

const SAMPLE_1: &'static str = r#"
[fedora]
name=Fedora $releasever - $basearch
#baseurl=http://download.example/pub/fedora/linux/releases/$releasever/Everything/$basearch/os/
metalink=https://mirrors.fedoraproject.org/metalink?repo=fedora-$releasever&arch=$basearch
enabled=1
countme=1
metadata_expire=7d
repo_gpgcheck=0
type=rpm
gpgcheck=1
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-fedora-$releasever-$basearch
skip_if_unavailable=False
"#;


const SAMPLE_2: &'static str = r#"
[nexus-production]
name=Production Repository
baseurl=http://localhost:8081/nexus/service/local/yum/repos/releases/production/
enabled=1
protect=0
gpgcheck=0
metadata_expire=30s
autorefresh=1
type=rpm-md
Promote RPM through Stages "#;


#[test]
fn test_name() {
    
}
}
