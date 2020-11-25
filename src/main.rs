extern crate hex;

#[macro_use]
extern crate lazy_static;

mod features;

use multihash::{Code};
use multihash::MultihashDigest;
use tide::Request;
use tide::Response;
use tide::StatusCode;
use std::sync::{Arc, Mutex};
use rusty_leveldb::DB;

lazy_static! {
    static ref DB_CONN: Arc<Mutex<DB>> = Arc::new(Mutex::new(features::db::get_database().unwrap()));
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/:hx").get(get_item);
    app.at("/").post(add_item);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_item(req: Request<()>) -> tide::Result {
    let hx: &str = req.param("hx")?;
    log::info!("getting: {:#?}", hx);

    let bytes = match hex::decode(hx) {
        Err(e) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_error(e);
            return Ok(resp);
        }
        val => val,
    }?;

    let mut db = DB_CONN.lock().unwrap();
    let res = features::db::read(&mut db, &bytes);
    match res {
        Some(v) => {
            let deserialized: features::Person = serde_cbor::from_slice(&v).unwrap();
            Ok(format!("{:#?}", deserialized).into())
        }
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}

async fn add_item(mut req: Request<()>) -> tide::Result {
    let person: features::Person = req.body_json().await?;
    let serialized = serde_cbor::to_vec(&person)?;
    let hash = Code::Keccak224.digest(&serialized);
    let digest = hash.digest();
    log::info!("putting: {:#?}", hex::encode(digest));
    let mut db = DB_CONN.lock().unwrap();
    features::db::store(&mut db, digest, &serialized)?;
    Ok(format!("{}", hex::encode(digest)).into())
}
