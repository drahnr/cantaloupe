use super::*;

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

