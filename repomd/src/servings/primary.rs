use super::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::integration::assets::repo;
    use crate::integration::groundtruth;

    #[test]
    fn primary() {
        let x = repo();

        let primary = Primary::new(&x);
        let content = primary.xml_render().expect("No reason to fail rendering xml. qed");
        assert_eq!(content.replace('\n', ""), dbg!(groundtruth::primary_xml()).replace('\n', ""));
    }
}