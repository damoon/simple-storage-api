extern crate hex;

mod features;

use multihash::Keccak224;
use tide::Request;

#[allow(dead_code)]
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
    let hx: &str = req.param("hx").unwrap_or("0");
    let bytes = hex::decode(hx).unwrap();
    let mut db = features::db::get_database();
    let res = features::db::read(&mut db, &bytes);
    let deserialized: features::Person = serde_cbor::from_slice(&res).unwrap();
    Ok(format!("{:#?}", deserialized).into())
}

async fn add_item(mut req: Request<()>) -> tide::Result {
    let person: features::Person = req.body_json().await?;
    let serialized = serde_cbor::to_vec(&person).unwrap();

    let hash = Keccak224::digest(&serialized);
    let mut db = features::db::get_database();
    features::db::store(&mut db, &hash, &serialized);

    Ok(format!("{}", hex::encode(hash)).into())
}
