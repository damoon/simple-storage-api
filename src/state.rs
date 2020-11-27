use rocksdb::{Error, IteratorMode, DB};
use std::path::Path;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::vec::Vec;

pub fn get_database() -> DB {
    log::info!("opening database");

    let path = Path::new("./.db");
    DB::open_default(path).unwrap()
}

pub fn store(db: &mut RwLockWriteGuard<DB>, key: &[u8], value: &[u8]) -> Result<(), Error> {
    db.put(key, value)?;
    db.flush()?;
    Ok(())
}

pub fn read(db: &RwLockReadGuard<DB>, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    db.get(key)
}

pub fn list(db: &RwLockReadGuard<DB>) -> Vec<Vec<u8>> {
    let mut vec: Vec<Vec<u8>> = Vec::new();
    db.full_iterator(IteratorMode::Start)
        .for_each(|(i, _x)| vec.push(i.into_vec()));
    vec
}

pub fn delete(db: &mut RwLockWriteGuard<DB>, key: &[u8]) -> Result<(), Error> {
    db.delete(key)
}
