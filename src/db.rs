include!(concat!(env!("OUT_DIR"), "/ongaku.db.rs"));

use std::{
    fs::{exists, File},
    io::{BufReader, Read, Write},
};

use prost::Message;

use crate::error::OngakuError;

static DB_FILE: &str = ".ongaku.pb";
static DB_VERSION: u32 = 1;

pub fn init() -> Result<(), OngakuError> {
    let library = Library {
        version: DB_VERSION,
        entry: Vec::new(),
    };

    let mut db_file = File::create_new(DB_FILE).map_err(|_| OngakuError::AlreadyInitialized)?;

    let mut db_buf = Vec::new();
    library.encode(&mut db_buf)?;
    db_file.write_all(&db_buf)?;

    Ok(())
}

pub fn load() -> Result<Library, OngakuError> {
    exists(DB_FILE).map_err(|_| OngakuError::NotInitialized)?;

    let db_file = File::open(DB_FILE)?;
    let mut buf_reader = BufReader::new(db_file);
    let mut db_buf = String::new();
    buf_reader.read_to_string(&mut db_buf)?;

    let library = Library::decode(db_buf.as_bytes())?;

    Ok(library)
}

pub fn save(library: Library) -> Result<(), OngakuError> {
    todo!("implement save")
}
