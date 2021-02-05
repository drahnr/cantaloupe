use super::*;

#[derive(Debug)]
pub struct Other<'a> {
    repo: &'a Repo,
}

impl<'a> Other<'a> {
    pub fn new(repo: &'a Repo) -> Self {
        Self { repo }
    }
}

impl<'a> XmlRender for Other<'a> {
    fn xml_render(&self) -> Result<String> {
        Ok(format!(
            r#"
<?xml version="1.0" encoding="UTF-8"?>
<otherdata xmlns="http://linux.duke.edu/metadata/other" packages="{}">
</otherdata>"#,
            self.repo.count_packages()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_xml_eq;
    use crate::integration::assets::repo;
    use crate::integration::groundtruth;
    #[test]
    fn other() {
        let x = repo();

        let other = Other::new(&x);
        let content = other
            .xml_render()
            .expect("No reason to fail rendering xml. qed");
        assert_xml_eq!(content, groundtruth::other_xml());
    }
}
