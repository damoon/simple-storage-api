extern crate leveldb;

use leveldb::database::Database;
//use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::path::Path;

pub fn get_database() -> leveldb::database::Database<i32> {
    let path = Path::new("./.db");
    let mut options = Options::new();
    options.create_if_missing = true;
    //    options.sync = true;
    let database: Database<i32> = match Database::open(path, options) {
        Ok(db) => db,
        Err(e) => panic!("failed to open database: {:?}", e),
    };
    database
}
pub fn store(db: &Database<i32>, key: &i32, value: &[u8]) {
    let mut write_opts = WriteOptions::new();
    write_opts.sync = true;
    match db.put(write_opts, key, value) {
        Ok(_) => (),
        Err(e) => panic!("failed to write to database: {:?}", e),
    };
}

pub fn read(db: &Database<i32>, key: &i32) -> Vec<u8> {
    let read_opts = ReadOptions::new();
    let res = db.get(read_opts, key);
    let data: Vec<u8> = match res {
        Ok(d) => {
            assert!(d.is_some());
            d.unwrap()
            //assert_eq!(data, Some(vec![1]));
        }
        Err(e) => panic!("failed reading data: {:?}", e),
    };
    data
}
