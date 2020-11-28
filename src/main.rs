extern crate hex;

use http_types::Body;
use multihash::Code;
use multihash::MultihashDigest;
use rocksdb::DB;
use std::env;
use std::sync::{Arc, RwLock};
use tide::Request;
use tide::Response;
use tide::StatusCode;
use tide::Server;
use serde::Serialize;
use serde::de::DeserializeOwned;

mod state;
mod people;
mod todos;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let cfs = vec![
        std::any::type_name::<people::Person>(),
        std::any::type_name::<todos::Task>(),
    ];

    let locked_db = Arc::new(RwLock::new(state::get_database(cfs)));
    let mut app = tide::with_state(locked_db);

    add_handlers::<people::Person>(&mut app, "people");
    add_handlers::<todos::Task>(&mut app, "todos");

    app.listen(listen_address()).await?;
    Ok(())
}

fn add_handlers<T: 'static + Serialize + DeserializeOwned>(app: &mut Server<Arc<RwLock<DB>>>, path: &str) {
    app.at(format!("/{}/:hx", path).as_str()).get(get_item::<T>);
    app.at(format!("/{}/:hx", path).as_str()).delete(delete_item::<T>);
    app.at(format!("/{}/", path).as_str()).post(add_item::<T>);
    app.at(format!("/{}/", path).as_str()).get(list_items::<T>);
}

fn listen_address() -> String {
    match env::var("TIDE_ADDR") {
        Ok(val) => val,
        Err(_e) => "127.0.0.1:8080".to_string(),
    }
}

async fn get_item<T: Serialize + DeserializeOwned>(req: Request<Arc<RwLock<DB>>>) -> tide::Result {
    let hx: &str = req.param("hx")?;

    let bytes = match hex::decode(hx) {
        Err(e) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_error(e);
            return Ok(resp);
        }
        val => val,
    }?;

    let prefix = std::any::type_name::<T>();
    let db = req.state().read().unwrap();
    let res = state::read(&db, prefix, &bytes);
    match res {
        Ok(Some(v)) => {
            let item: T = serde_cbor::from_reader(&v[..]).unwrap();
            let mut resp = Response::new(StatusCode::Ok);
            resp.set_body(Body::from_json(&item)?);
            resp.set_content_type("application/json");
            Ok(resp)
        }
        Ok(None) => Ok(Response::new(StatusCode::NotFound)),
        Err(e) => {
            let mut resp = Response::new(StatusCode::InternalServerError);
            resp.set_error(e);
            Ok(resp)
        }
    }
}

async fn add_item<T: Serialize + DeserializeOwned>(mut req: Request<Arc<RwLock<DB>>>) -> tide::Result {
    let item: T = req.body_json().await?;
    let serialized = serde_cbor::to_vec(&item)?;
    let hash = Code::Keccak224.digest(&serialized);
    let digest = hash.digest();

    let prefix = std::any::type_name::<T>();
    let mut db = req.state().write().unwrap();
    state::store(&mut db, prefix, digest, &serialized)?;

    let mut resp = Response::new(StatusCode::Created);
    resp.set_body(Body::from_json(&hex::encode(digest))?);
    resp.set_content_type("application/json");
    Ok(resp)
}

async fn delete_item<T>(req: Request<Arc<RwLock<DB>>>) -> tide::Result {
    let hx: &str = req.param("hx")?;

    let bytes = match hex::decode(hx) {
        Err(e) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_error(e);
            return Ok(resp);
        }
        val => val,
    }?;

    let prefix = std::any::type_name::<T>();
    let mut db = req.state().write().unwrap();
    state::delete(&mut db, prefix, &bytes)?;
    Ok(Response::new(StatusCode::NoContent))
}

async fn list_items<T>(req: Request<Arc<RwLock<DB>>>) -> tide::Result {
    let prefix = std::any::type_name::<T>();
    let db = req.state().read().unwrap();
    let vec = state::list(&db, prefix);

    let mut ls: Vec<String> = Vec::new();
    for v in vec {
        let s = hex::encode(v);
        ls.push(s)
    }

    let body = Body::from_json(&ls)?;
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(body);
    return Ok(resp);
}
