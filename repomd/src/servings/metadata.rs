use super::*;

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
        let other = Other::new(self.repo);

        Ok(format!(
r#"
<?xml version="1.0" encoding="UTF-8"?>
<repomd xmlns="{url}">
<data type="primary">
    <open-checksum type="{digest_name}">{primary_uncompressed_digest}</open-checksum>
    <checksum type="{digest_name}">{primary_digest}</checksum>
    <location href="repodata/{primary_digest}-primary.xml.gz"/>
    <timestamp>{timestamp}</timestamp>
</data>
<data type="filelists">
    <open-checksum type="{digest_name}">{filelist_uncompressed_digest}</open-checksum>
    <checksum type="{digest_name}">{filelist_digest}</checksum>
    <location href="repodata/{filelist_digest}-filelists.xml.gz"/>
    <timestamp>{timestamp}</timestamp>
</data>
<data type="other">
    <open-checksum type="{digest_name}">{filelist_uncompressed_digest}</open-checksum>
    <checksum type="{digest_name}">{other_digest}</checksum>
    <location href="repodata/{other_digest}-other.xml.gz"/>
    <timestamp>{timestamp}</timestamp>
</data>
"#,
    url = self.repo.url().as_str(),
    timestamp = self.timestamp(),
    digest_name = primary.digest_name(),
    primary_digest = primary.compressed_xml_digest()?,
    primary_uncompressed_digest = primary.xml_digest()?,
    filelist_digest = filelist.compressed_xml_digest()?,
    filelist_uncompressed_digest = filelist.xml_digest()?,
    other_digest = other.compressed_xml_digest()?,
    other_uncompressed_digest = other.xml_digest()?,
))
    }
}
