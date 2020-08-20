use std::collections::HashMap;

use actix_web::{
    put,get,post,
    middleware,
    error, http,
    web::{self, Query},
    App, Error, HttpResponse, HttpServer, Responder
};

use bytes::BytesMut;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use std::str::FromStr;

use std::sync::Mutex;
use std::sync::RwLock;
use rpm;
use std::io::{Read};
use anyhow::Context;

const MAX_RPM_SIZE: usize = 128_000_000; // max payload size is 128 MB

mod errors;

use repo::{self, *};

mod xml;
use crate::xml::*;


#[get("/repo.xml")]
fn repo_xml() -> HttpResponse {
    let s = format!("<TO><DO></DO></TO>");
    HttpResponse::Ok().content_type("text/html").body(s)
}

/// Query what
#[get("/index.html")]
fn index(
    state: web::Data<SharedState>,
    query: Query<HashMap<String, String>>) -> HttpResponse {


    let repo = state.repo.read().unwrap();

    let s = if let Some(_name) = query.get("name") {
        let meta_data = RepoMetaData::new(&repo);
        meta_data.xml_render().unwrap()
    } else {
        Index::new(&repo).xml_render().unwrap()
    };
    HttpResponse::Ok().content_type("text/html").body(s)
}

// fn stream_pkg(name : &str, data : &Bytes) -> Result<HttpResponse> {

//     let mut stream = redis::Cmd::get(name).query_async(&mut redis_con).await.and_then(|bytes| {
//         bytes.stream()
//     }).or_else(|| {
//         // read from postgres db
//     });

//     Ok(HttpResponse::Ok().content_type("application/x-rpm").content_length(data.len()).streaming(stream))
// }

#[derive(Debug)]
pub struct SharedState {
    pub verifier: Mutex<rpm::signature::pgp::Verifier>,
    pub repo: RwLock<Repo>,
}

impl SharedState {
    pub fn new() -> Self {
        const x: &'static [u8] = include_bytes!("../public_key.asc");
        // FIXME TODO
        Self {
            verifier : Mutex::new(rpm::signature::pgp::Verifier::load_from_asc_bytes(&x).unwrap() ),
            repo: RwLock::new(Repo::new( "https://canta.ahoi.io".parse().unwrap() )),
        }
    }

    pub fn add_verification_key<R: Read>(_rdr: R) -> Result<(), Error> {
        unimplemented!("nope")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    link: url::Url,
    version: String,
    verified: bool,
}

use std::string::ToString;

#[put("/index.html")]
async fn upload_rpm(
    state: web::Data<SharedState>,
    mut payload: web::Payload,
) -> HttpResponse {
    log::info!("Starting upload");
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_RPM_SIZE {
            log::warn!("RPM too large")
            //return Err(error::ErrorBadRequest("rpm too large"));
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

    HttpResponse::Ok().json(x)
}

#[get("/alive/{whatever}")]
async fn alive(info: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().content_type("plain/text").body(format!("Hello {}!", info))
}


fn main() -> errors::Result<()> {
    env_logger::init();

    let mut sys = actix_rt::System::new("cantaloupe");

    // start http server
    let http = HttpServer::new(move || {
        App::new()
            .data(SharedState::new())
            .service(upload_rpm)
            .service(index)
            .service(alive )
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8899")?
    .workers(8);


    log::info!("Started http server: 127.0.0.1:8899");
    let server = http.run();
    let _ = sys.block_on(server);
    Ok(())
}
