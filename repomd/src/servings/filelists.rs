use crate::error::Result;
use crate::xml::XmlRender;
use crate::Repo;

#[derive(Debug)]
pub struct FileList<'a> {
    repo: &'a Repo,
}

impl<'a> FileList<'a> {
    pub fn new(repo: &'a Repo) -> Self {
        Self { repo }
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
            r#"<?xml version="1.0" encoding="UTF-8"?>
<filelists xmlns="http://linux.duke.edu/metadata/filelists" packages="{package_count}">
"#,
            package_count = package_count
        );
        s.push_str(hdr_s.as_str());

        for (_, _, pkg) in self.repo.packages() {
            let pkg_s = format!(
                r#"<package pkgid="{identifier}" name="{name}" arch="{arch}">
  <version epoch="{epoch}" ver="{version}" rel="{release}" />
"#,
                identifier = pkg.identifier(),
                name = pkg.name(),
                arch = pkg.arch(),
                epoch = pkg.epoch(),
                version = pkg.version(),
                release = pkg.release()
            );
            s.push_str(pkg_s.as_str());
            for file in pkg.files() {
                let file_s = format!(r#"    <file>{path}</file>"#, path = file.display());
                s.push_str(file_s.as_str());
            }
            s.push_str(r#"</package>"#);
        }
        s.push_str(r#"</filelists>"#);
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_xml_eq;
    use crate::integration::assets::repo;
    use crate::integration::groundtruth;

    #[test]
    fn filelists() {
        let x = repo();

        let fl = FileList::new(&x);
        let content = fl
            .xml_render()
            .expect("No reason to fail rendering xml. qed");
        assert_xml_eq!(content, groundtruth::filelists_xml());
    }
}
