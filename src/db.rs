include!(concat!(env!("OUT_DIR"), "/ongaku.db.rs"));

use std::{
    collections::HashMap,
    fs::{exists, File},
    io::{BufReader, Read, Write},
};

use log::info;
use prost::Message;

use crate::error::OngakuError;

static DB_FILE: &str = ".ongaku.pb";
static DB_VERSION: u32 = 1;

pub fn init() -> Result<(), OngakuError> {
    info!("Creating new library");
    let library = Library {
        version: DB_VERSION,
        entries: HashMap::new(),
    };

    info!("Creating library file");
    let mut db_file = File::create_new(DB_FILE).map_err(|_| OngakuError::AlreadyInitialized)?;

    info!("Encoding library");
    let mut db_buf = Vec::new();
    library.encode(&mut db_buf)?;

    info!("Writing library to file");
    Ok(db_file.write_all(&db_buf)?)
}

pub fn load() -> Result<Library, OngakuError> {
    info!("Checking library file exists");
    exists(DB_FILE).map_err(|_| OngakuError::NotInitialized)?;

    info!("Reading library file");
    let db_file = File::open(DB_FILE)?;
    let mut buf_reader = BufReader::new(db_file);
    let mut db_buf: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut db_buf)?;

    info!("Decoding library");
    Ok(Library::decode(&*db_buf)?)
}

pub fn save(library: Library) -> Result<(), OngakuError> {
    info!("Checking library file exists");
    exists(DB_FILE).map_err(|_| OngakuError::NotInitialized)?;

    info!("Encoding library");
    let db_buf = library.encode_to_vec();

    info!("Writing library file");
    let mut db_file = File::create(DB_FILE)?;
    Ok(db_file.write_all(&db_buf)?)
}
