use rusty_leveldb::{Options, DB};

use std::path::Path;
use std::vec::Vec;

pub fn get_database() -> DB {
    let path = Path::new("./.db");
    let mut options = Options::default();
    options.create_if_missing = true;

    let db = DB::open(&path, options).unwrap();
    db
}
pub fn store(db: &mut DB, key: &[u8], value: &[u8]) {
    db.put(key, value).unwrap();
}

pub fn read(db: &mut DB, key: &[u8]) -> Vec<u8> {
    let res = db.get(key).unwrap();
    res.as_slice().to_vec()
}
