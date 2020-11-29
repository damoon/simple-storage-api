use sled::{Result, Db, IVec};
use std::path::Path;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::vec::Vec;

pub fn get_database(_cfs: Vec<&str>) -> Db {
    log::info!("opening database");

    let path = Path::new("./.db");
    sled::open(&path).unwrap()
}

pub fn store(db: &mut RwLockWriteGuard<Db>, prefix: &str, key: &[u8], value: &[u8]) -> Result<()> {
    db.open_tree(prefix)?.insert(key, value)?;
    db.flush()?;
    Ok(())
}

pub fn read(db: &RwLockReadGuard<Db>, prefix: &str, key: &[u8]) -> Result<Option<IVec>> {
    db.open_tree(prefix)?.get(key)
}

pub fn list(db: &RwLockReadGuard<Db>, prefix: &str) -> Result<Vec<Vec<u8>>> {
    let mut keys: Vec<Vec<u8>> = Vec::new();
    let tree = db.open_tree(prefix)?;
    for res in tree.iter() {
        let (k, _) = res?;
        keys.push(k.to_vec());
    }
    Ok(keys)
}

pub fn delete(db: &mut RwLockWriteGuard<Db>, prefix: &str, key: &[u8]) -> Result<Option<IVec>> {
    db.open_tree(prefix)?.remove(key)
}
