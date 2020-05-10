#[macro_use]
extern crate redis_async;
use serde::Deserialize;

use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::join_all;
use redis_async::resp::RespValue;

#[derive(Deserialize)]
pub struct CacheInfo {
    full_package_name: String,
}

async fn cache_stuff(
    info: web::Json<CacheInfo>,
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
    let info = info.into_inner();

    let one = redis.send(Command(resp_array!["SET", "mydomain:one", info.one]));
    let two = redis.send(Command(resp_array!["SET", "mydomain:two", info.two]));
    let three = redis.send(Command(resp_array!["SET", "mydomain:three", info.three]));

    // Creates a future which represents a collection of the results of the futures
    // given. The returned future will drive execution for all of its underlying futures,
    // collecting the results into a destination `Vec<RespValue>` in the same order as they
    // were provided. If any future returns an error then all other futures will be
    // canceled and an error will be returned immediately. If all futures complete
    // successfully, however, then the returned future will succeed with a `Vec` of
    // all the successful results.
    
    let good = join_all(vec![one, two, three].into_iter())
            .await
            .into_iter()
            .try_map(|item| {
                item.map_err(AWError::from)
                    .and_then(|res| res.map_err(AWError::from))
            })
            .all(|res| {
                match res {
                    Ok(RespValue::SimpleString(x)) if x == "OK" => true,
                    _ => false,
                }
            });

    // successful operations return "OK", so confirm that all returned as so
    if good {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    }
}

async fn delete_cache_entry_stuff(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let res = redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .await?;

    match res {
        Ok(RespValue::Integer(x)) if x == 3 => {
            Ok(HttpResponse::Ok().body("successfully deleted values"))
        }
        _ => {
            println!("---->{:?}", res);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}