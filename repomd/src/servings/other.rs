use super::*;

#[derive(Debug)]
pub struct Other<'a> {
    repo: &'a Repo,
}

impl<'a> Other<'a> {
    fn new(repo: &'a Repo) -> Self {
        Self {
            repo,
        }
    }
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
