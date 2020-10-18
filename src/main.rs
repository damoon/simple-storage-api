mod features;

use multihash::Keccak224;
use std::env;
use std::str::FromStr;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let role = features::Role::from_str(&args[3]).unwrap();

    let person = features::Person {
        name: args[1].clone(),
        age: args[2].parse::<u8>().unwrap(),
        role: role,
    };

    let serialized = serde_cbor::to_vec(&person).unwrap();
    let hash = Keccak224::digest(&serialized);
    let mut db = features::db::get_database();
    features::db::store(&mut db, &hash, &serialized);

    let res = features::db::read(&mut db, &hash);
    let deserialized: features::Person = serde_cbor::from_slice(&res).unwrap();

    println!("{:#?}", deserialized);

    features::output::show_person(deserialized);
}
