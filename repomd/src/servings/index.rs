use super::*;

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

