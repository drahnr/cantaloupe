use std::convert::TryInto;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use which;
pub(crate) mod helper {
    use super::*;

    /// launch crate repo
    pub(crate) fn launch_createrepo() {
        let createrepo =
            which::which("createrepo").expect("Did not find createrepo in search paths");
        let manifestdir = env!("CARGO_MANIFEST_DIR");
        let outdir = PathBuf::from(manifestdir).join("groundtruth");
        let indir = PathBuf::from(manifestdir).join("tests").join("assets");
        let baseurl = "https://repo.konifay.io/cantaloupe/";

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

#[macro_export]
macro_rules! assert_xml_eq {
    ($left:expr, $right:expr) => {
        let left: String = $left;
        let right: String = $right;

        let normalize = |xml: String| -> String {
            use quick_xml::events::Event;
            use quick_xml::Reader;
            use quick_xml::Writer;
            use std::io::Cursor;

            let mut reader = Reader::from_str(xml.as_str());
            reader.trim_text(true);
            let mut writer = Writer::new(Cursor::new(Vec::new()));
            let mut buf = Vec::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Eof) => break,
                    Ok(event) => assert!(writer.write_event(event).is_ok()),
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                }
                buf.clear();
            }

            let v = writer.into_inner().into_inner();
            let cow = String::from_utf8_lossy(v.as_slice());
            let s: &str = &cow;
            s.to_owned()
        };

        let left = normalize(left);
        let right = normalize(right);
        assert_eq!(left, right);
    };
}

pub(crate) mod groundtruth {
    use compression::prelude::*;

    pub(crate) fn repomd_xml() -> String {
        let data = include_bytes!("../groundtruth/repodata/repomd.xml");
        let s = std::string::String::from_utf8_lossy(data);
        let s: &str = &s;
        let s = s.to_owned();
        s
    }

    pub(crate) fn primary_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/81f452c5f8139c65da555d19dad7adeae3f4e2e3665ef538243ac161862b3885-primary.xml.gz")
    }

    pub(crate) fn filelists_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/9d2b70627231fc3d6abc0c6e41a425b2a888213a30745acffb2b670b5727a12f-filelists.xml.gz")
    }

    pub(crate) fn other_xml_gz() -> &'static [u8] {
        include_bytes!("../groundtruth/repodata/0f12187e68182b9f05d922d8f480316e78da5ce7e3b028b31c53e9d373d8b6d4-other.xml.gz")
    }

    fn decompressed(data: &'static [u8]) -> String {
        let x = data
            .into_iter()
            .cloned()
            .decode(&mut GZipDecoder::new())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        String::from(std::string::String::from_utf8_lossy(x.as_slice()))
    }

    pub(crate) fn filelists_xml() -> String {
        decompressed(filelists_xml_gz())
    }
    pub(crate) fn primary_xml() -> String {
        decompressed(primary_xml_gz())
    }
    pub(crate) fn other_xml() -> String {
        decompressed(other_xml_gz())
    }
}

pub(crate) mod assets {
    use super::*;
    use crate::Arch;
    use crate::Package;
    use crate::Release;
    use crate::Repo;
    use crate::Version;
    use digest::Digest;
    use std::path::PathBuf;
    use url::Url;

    /// Reads a full rpm from `data`.
    fn read_rpm(data: &'static [u8]) -> Package {
        let mut buf_reader = std::io::BufReader::new(data);
        let pkg =
            rpm::RPMPackage::parse(&mut buf_reader).expect("Must be able to create reader. qed");
        // @todo we do not have the keys ready just yet
        let header = pkg.metadata.header;

        let files = header
            .get_file_entries()
            .map(|entries| {
                entries
                    .into_iter()
                    .map(|entry| entry.path)
                    .collect::<Vec<PathBuf>>()
            })
            .expect("PackageHeader must have required file fields. qed");

        let mut hasher = sha2::Sha256::new();
        hasher.update(data);

        let pkg = Package {
            identifier: hex::encode(hasher.finalize().as_slice()),
            name: header
                .get_name()
                .expect("PackageHeader must have >name< field. qed")
                .to_owned(),
            version: Version::from_str(
                header
                    .get_version()
                    .expect("PackageHeader must have >version< field. qed"),
            )
            .unwrap(),
            epoch: header
                .get_epoch()
                .unwrap_or_default()
                .try_into()
                .unwrap_or_default(),
            release: Release::from_str(
                header
                    .get_release()
                    .expect("PackageHeader must have >release< field. qed"),
            )
            .unwrap(),
            // @todo this is actully `7.fc32`
            //u32::from_str(dbg!(
            //    .expect("Must be a str"),
            arch: Arch::from_str(
                header
                    .get_arch()
                    .expect("PackageHeader must have >arch< field. qed"),
            )
            .unwrap(),
            files,
        };
        pkg
    }

    pub(crate) fn rpm_pkg_1() -> (&'static [u8], Package) {
        const DATA: &'static [u8] = include_bytes!("../tests/assets/vlc-3.0.9.2-3.fc32.x86_64.rpm");
        (DATA, read_rpm(DATA))
    }

    pub(crate) fn rpm_pkg_2() -> (&'static [u8], Package) {
        const DATA: &'static [u8] =
            include_bytes!("../tests/assets/wf-recorder-0.2.1-1.fc32.x86_64.rpm");
        (DATA, read_rpm(DATA))
    }

    pub(crate) fn repo() -> crate::Repo {
        let mut repo = Repo::new(Url::parse("https://repo.konifay.io/cantaloupe").unwrap());

        repo.add_package(rpm_pkg_1().1);
        repo.add_package(rpm_pkg_2().1);

        repo
    }
}

#[test]
fn cmp_to_create_repo() {
    helper::launch_createrepo();
}
