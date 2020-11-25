use rusty_leveldb::{Options, DB};
use std::path::Path;
use std::vec::Vec;

pub fn get_database() -> Result<DB, rusty_leveldb::Status>{
    log::info!("opening database");
    let path = Path::new("./.db");
    let mut options = Options::default();
    options.create_if_missing = true;
    DB::open(&path, options)
}
pub fn store(db: &mut DB, key: &[u8], value: &[u8]) -> Result<(), rusty_leveldb::Status>{
    db.put(key, value)?;
    db.flush()?;
    Ok(())
}

pub fn read(db: &mut DB, key: &[u8]) -> Option<Vec<u8>> {
    db.get(key)
}
