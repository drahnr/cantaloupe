use std::process::Command;
use which;
use std::env;
use std::path::PathBuf;


pub(crate) mod helper {
    use super::*;

    pub(crate) fn launch_createrepo() {
        let createrepo = which::which("createrepo").expect("Did not find createrepo in search paths");
        let manifestdir = env!("CARGO_MANIFEST_DIR");
        let outdir = PathBuf::from(manifestdir).join("groundtruth");
        let indir = PathBuf::from(manifestdir).join("tests").join("assets");
        let baseurl = "http://sub.example.com/repo/";

        let output = Command::new(createrepo)
            .arg(format!("--outputdir={}", outdir.to_string_lossy()))
            .arg(format!("--baseurl={}", baseurl))
            .arg("--verbose")
            .arg("--pretty")
            .arg("--no-database")
            .arg("--checksum=sha256")
            .arg("--workers=2")
            .arg(indir.as_os_str())
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let stderr = String::from_utf8_lossy(output.stderr.as_slice());

        println!("> {}", stdout);
        eprintln!("> {}", stderr);
        assert!(output.status.success());
    }
}

pub(crate) mod groundtruth {


    pub(crate) fn repomod_xml() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/repomd.xml")
    }

    pub(crate) fn primary_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/349b990aa8a8c9a8ad1b85866a00e8a8b87d9e2b66e7e10491f8d189621a41ea-primary.xml.gz")
    }

    pub(crate) fn filelists_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/9d2b70627231fc3d6abc0c6e41a425b2a888213a30745acffb2b670b5727a12f-filelists.xml.gz")
    }

    pub(crate) fn other_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/0f12187e68182b9f05d922d8f480316e78da5ce7e3b028b31c53e9d373d8b6d4-other.xml.gz")
    }
}

pub(crate) mod assets {

    pub(crate) fn other_xml_gz() -> &'static [u8] {
        include_bytes!("../tests/assets/vlc-3.0.9.2-3.fc32.x86_64.rpm")
    }

    pub(crate) fn rpm() -> &'static [u8] {
        include_bytes!("../tests/assets/wf-recorder-0.2.1-1.fc32.x86_64.rpm")
    }
}


#[test]
fn cmp_to_create_repo() {
    helper::launch_createrepo();
}