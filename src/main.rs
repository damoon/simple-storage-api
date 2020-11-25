extern crate hex;

mod features;

use multihash::Keccak224;
use tide::Request;
use tide::Response;
use tide::StatusCode;

#[allow(dead_code)]
#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/:hx").get(get_item);
    app.at("/").post(add_item);
    app.listen("127.0.0.1:8081").await?;
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

    let mut db = features::db::get_database()?;
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
    let hash = Keccak224::digest(&serialized);
    log::info!("putting: {:#?}", hex::encode(&hash));
    let mut db = features::db::get_database()?;
    features::db::store(&mut db, &hash, &serialized)?;
    Ok(format!("{}", hex::encode(hash)).into())
}
