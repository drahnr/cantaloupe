use std::collections::HashMap;

use actix;
use actix_web::{
    error, http,
    web::{self, Query},
    App, Error, HttpResponse, HttpServer,
};
use bytes::BytesMut;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use rpm;
use std::io::{Read,Write,Cursor};
use anyhow::Context;

const MAX_RPM_SIZE: usize = 128_000_000; // max payload size is 128 MB

mod errors;

mod repo;
use crate::repo::*;

mod xml;
use crate::xml::*;

fn repo_xml() -> Result<HttpResponse, Error> {
    unimplemented!()
}

/// Query what
fn index(
    state: web::Data<SharedState>,
    query: Query<HashMap<String, String>>) -> Result<HttpResponse,Error> {


    let repo = state.repo.read().unwrap();

    let s = if let Some(name) = query.get("name") {
        let meta_data = RepoMetaData::new(&repo);
        meta_data.xml_render().unwrap()
    } else {
        Index::new(&repo).xml_render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// fn stream_pkg(name : &str, data : &Bytes) -> Result<HttpResponse> {

//     let mut stream = redis::Cmd::get(name).query_async(&mut redis_con).await.and_then(|bytes| {
//         bytes.stream()
//     }).or_else(|| {
//         // read from postgres db
//     });

//     Ok(HttpResponse::Ok().content_type("application/x-rpm").content_length(data.len()).streaming(stream))
// }

#[derive(Clone, Debug)]
pub struct SharedState {
    pub verifier: Arc<Mutex<rpm::signature::pgp::Verifier>>,
    pub repo: Arc<RwLock<Repo>>,
}

impl SharedState {
    pub fn new() -> Self {
        const x: &'static [u8] = include_bytes!("../public_key.asc");
        // FIXME TODO
        Self {
            verifier : Arc::new(Mutex::new(rpm::signature::pgp::Verifier::load_from_asc_bytes(&x).unwrap() )),
            repo: Arc::new( RwLock::new(Repo::new( "canta.ahoi.io".parse().unwrap() ))),
        }
    }

    pub fn add_verification_key<R: Read>(rdr: R) -> Result<(), Error> {
        unimplemented!("nope")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    link: url::Url,
    version: String,
    verified: bool,
}

async fn upload_rpm(
    state: web::Data<SharedState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_RPM_SIZE {
            return Err(error::ErrorBadRequest("rpm too large"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut cursor = std::io::Cursor::new(&body);
    let package = rpm::RPMPackage::parse(&mut cursor)
        .with_context(|| "Uploaded RPM  failed format check")
        .unwrap();

    {
        let guard = state.verifier.lock().unwrap();
        package
            .verify_signature(&*guard)
            .with_context(|| "Failed signature check")
            .unwrap();
    }

    let x = UploadResponse {
        link: url::Url::from_str("https://ahoi.io").unwrap(),
        version: package.metadata.header.get_version().unwrap().to_string(),
        verified: true,
    };

    Ok(HttpResponse::Ok().json(x)) // <- send response
}

fn main() -> errors::Result<()> {
    let sys = actix::System::new("cantaloupe");

    // start http server
    let http = HttpServer::new(move || {
        App::new()
            .app_data(SharedState::new())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/repo.xml")
                .route(web::get().to(repo_xml))
                .route( web::post().to(upload_rpm)))
    })
    .bind("127.0.0.1:8899")?
    .workers(8);


    println!("Started http server: 127.0.0.1:8899");
    sys.block_on(http)?;
    Ok(())
}
