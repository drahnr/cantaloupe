use sha2::{Digest,Sha256};
use bytes::Bytes;

// use std::marker::PhantomData;

use compression::prelude::*;


use crate::error::{Error, Result};

pub trait XmlRender {
    fn xml_render(&self) -> Result<String>;


    fn compressed_xml_bytes(&self) -> Result<Bytes> {
        let s = self.xml_render()?;
        let bytes = s.as_bytes()
            .into_iter()
            .cloned()
            .encode(&mut GZipEncoder::new(), Action::Finish)
            .collect::<std::result::Result<Vec<_>, _>>().map_err(|_err| {
                Error::CompressionFailed //.with_context(err)
            })?;
        Ok(Bytes::from(bytes))
    }

    fn xml_digest(&self) -> Result<String> {
        self.xml_render().and_then(|s| {
            let mut digest = Sha256::new();
            digest.update(s.as_bytes());
            let digest = digest.finalize();
            let digest_str = format!("{:x}", digest);
            Ok(digest_str)
        })
    }

    fn compressed_xml_digest(&self) -> Result<String> {
        self.compressed_xml_bytes().and_then(|bytes| {
            let mut digest = Sha256::new();
            digest.update(bytes);
            let digest = digest.finalize();

            let digest_str = format!("{:x}", digest);
            Ok(digest_str)
        })
    }


    fn digest_name(&self) -> String {
        "sha256".to_string()
    }

    fn compression_name() -> String {
       "gz".to_string()
    }
}

