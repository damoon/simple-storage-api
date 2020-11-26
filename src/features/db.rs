use rusty_leveldb::{Options, DB, LdbIterator};
use std::path::Path;
use std::vec::Vec;
use std::sync::MutexGuard;

pub fn get_database() -> Result<DB, rusty_leveldb::Status>{
    log::info!("opening database");
    let path = Path::new("./.db");
    let mut options = Options::default();
    options.create_if_missing = true;
    DB::open(&path, options)
}
pub fn store(db: &mut MutexGuard<DB>, key: &[u8], value: &[u8]) -> Result<(), rusty_leveldb::Status>{
    db.put(key, value)?;
    db.flush()?;
    Ok(())
}

pub fn read(db: &mut MutexGuard<DB>, key: &[u8]) -> Option<Vec<u8>> {
    db.get(key)
}

pub fn list(db: &mut MutexGuard<DB>) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    let mut it = db.new_iter().unwrap();
    let (mut k, mut v) = (vec![], vec![]);
    while it.advance() {
        it.current(&mut k, &mut v);
        vec.push(k.clone());
    }
    vec
}

pub fn delete(db: &mut MutexGuard<DB>, key: &[u8]) -> Result<(), rusty_leveldb::Status> {
    db.delete(key)
}
