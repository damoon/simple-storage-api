extern crate hex;

mod features;

use http_types::Body;
use multihash::Code;
use multihash::MultihashDigest;
use rusty_leveldb::DB;
use std::env;
use std::sync::{Arc, Mutex};
use tide::Request;
use tide::Response;
use tide::StatusCode;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let locked_db = Arc::new(Mutex::new(features::db::get_database().unwrap()));
    let mut app = tide::with_state(locked_db);
    app.at("/:hx").get(get_item);
    app.at("/:hx").delete(delete_item);
    app.at("/").post(add_item);
    app.at("/").get(list_items);
    app.listen(listen_address()).await?;
    Ok(())
}

fn listen_address() -> String {
    match env::var("TIDE_ADDR") {
        Ok(val) => val,
        Err(_e) => "127.0.0.1:8080".to_string(),
    }
}

async fn get_item(req: Request<Arc<Mutex<DB>>>) -> tide::Result {
    let hx: &str = req.param("hx")?;

    let bytes = match hex::decode(hx) {
        Err(e) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_error(e);
            return Ok(resp);
        }
        val => val,
    }?;

    let mut db = req.state().lock().unwrap();
    let res = features::db::read(&mut db, &bytes);
    match res {
        Some(v) => {
            let person: features::Person = serde_cbor::from_slice(&v).unwrap();
            let mut resp = Response::new(StatusCode::Ok);
            resp.set_body(Body::from_json(&person)?);
            resp.set_content_type("application/json");
            Ok(resp)
        }
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}

async fn add_item(mut req: Request<Arc<Mutex<DB>>>) -> tide::Result {
    let person: features::Person = req.body_json().await?;
    let serialized = serde_cbor::to_vec(&person)?;
    let hash = Code::Keccak224.digest(&serialized);
    let digest = hash.digest();

    let mut db = req.state().lock().unwrap();
    features::db::store(&mut db, digest, &serialized)?;

    let mut resp = Response::new(StatusCode::Created);
    resp.set_body(Body::from_json(&hex::encode(digest))?);
    resp.set_content_type("application/json");
    Ok(resp)
}

async fn delete_item(req: Request<Arc<Mutex<DB>>>) -> tide::Result {
    let hx: &str = req.param("hx")?;

    let bytes = match hex::decode(hx) {
        Err(e) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_error(e);
            return Ok(resp);
        }
        val => val,
    }?;

    let mut db = req.state().lock().unwrap();
    features::db::delete(&mut db, &bytes)?;
    Ok(Response::new(StatusCode::NoContent))
}

async fn list_items(req: Request<Arc<Mutex<DB>>>) -> tide::Result {
    let mut db = req.state().lock().unwrap();
    let vec = features::db::list(&mut db);

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
