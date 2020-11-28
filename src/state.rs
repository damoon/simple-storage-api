use rocksdb::{Error, IteratorMode, DB, Options, ColumnFamilyDescriptor};
use std::path::Path;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use std::vec::Vec;

pub fn get_database(cfs: Vec<&str>) -> DB {
    log::info!("opening database");

    let path = Path::new("./.db");
    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);

    let mut cfs_list: Vec<ColumnFamilyDescriptor> = Vec::new();
    for cf in cfs {
        let cf = ColumnFamilyDescriptor::new(cf, Options::default());
        cfs_list.push(cf);
    }

    DB::open_cf_descriptors(&db_opts, path, cfs_list).unwrap()
}

pub fn store(db: &mut RwLockWriteGuard<DB>, prefix: &str, key: &[u8], value: &[u8]) -> Result<(), Error> {
    let cf = db.cf_handle(prefix).unwrap();
    db.put_cf(cf, key, value)?;
    db.flush()?;
    Ok(())
}

pub fn read(db: &RwLockReadGuard<DB>, prefix: &str, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    let cf = db.cf_handle(prefix).unwrap();
    db.get_cf(cf, key)
}

pub fn list(db: &RwLockReadGuard<DB>, prefix: &str) -> Vec<Vec<u8>> {
    let cf = db.cf_handle(prefix).unwrap();
    let mut vec: Vec<Vec<u8>> = Vec::new();
    db.full_iterator_cf(cf, IteratorMode::Start)
        .for_each(|(i, _x)| vec.push(i.into_vec()));
    vec
}

pub fn delete(db: &mut RwLockWriteGuard<DB>, prefix: &str, key: &[u8]) -> Result<(), Error> {
    let cf = db.cf_handle(prefix).unwrap();
    db.delete_cf(cf, key)
}
