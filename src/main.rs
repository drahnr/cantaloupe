extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate common_failures;

use std::collections::HashMap;

use actix_web::{web, http, App, HttpServer, HttpResponse};
use actix_web::web::Query;

use common_failures::prelude::*;


mod errors;
mod repo;
use crate::repo::*;

mod xml;
use crate::xml::*;


fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let repo = Repo::new( "canta.ahoi.io".parse()? );

    let s = if let Some(name) = query.get("name") {

        let meta_data = RepoMetaData::new(&repo);
        meta_data.xml_render()?
    } else {
        Index::new(&repo).xml_render()?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn run() -> Result<()> {
    let sys = actix::System::new("cantaloupe");

    // start http server
    HttpServer::new(move || {
        App::new().service(
            web::resource("/").route(web::get().to(index))
        )
    }).bind("127.0.0.1:8899")?
        .start();

    println!("Started http server: 127.0.0.1:8899");
    sys.run()?;
    Ok(())
}



quick_main!(run);