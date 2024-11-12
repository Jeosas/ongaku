include!(concat!(env!("OUT_DIR"), "/ongaku.db.rs"));

use std::{
    collections::HashMap,
    fs::{exists, File},
    io::{BufReader, Read, Write},
};

use log::debug;
use prost::Message;

use crate::error::OngakuError;

static DB_FILE: &str = ".ongaku.pb";
static DB_VERSION: u32 = 1;

pub fn generate_id(id: &str, entry_type: &entry::EntryType) -> String {
    debug!("Generating new id");
    format!("{}/{}", entry_type.as_str_name(), id)
}

pub fn init() -> Result<(), OngakuError> {
    debug!("Creating new library");
    let library = Library {
        version: DB_VERSION,
        entries: HashMap::new(),
    };

    debug!("Creating library file");
    let mut db_file = File::create_new(DB_FILE).map_err(|_| OngakuError::AlreadyInitialized)?;

    debug!("Encoding library");
    let mut db_buf = Vec::new();
    library.encode(&mut db_buf)?;

    debug!("Writing library to file");
    Ok(db_file.write_all(&db_buf)?)
}

pub fn load() -> Result<Library, OngakuError> {
    debug!("Checking library file exists");
    exists(DB_FILE).map_err(|_| OngakuError::NotInitialized)?;

    debug!("Reading library file");
    let db_file = File::open(DB_FILE)?;
    let mut buf_reader = BufReader::new(db_file);
    let mut db_buf: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut db_buf)?;

    debug!("Decoding library");
    Ok(Library::decode(&*db_buf)?)
}

pub fn save(library: Library) -> Result<(), OngakuError> {
    debug!("Checking library file exists");
    exists(DB_FILE).map_err(|_| OngakuError::NotInitialized)?;

    debug!("Encoding library");
    let db_buf = library.encode_to_vec();

    debug!("Writing library file");
    let mut db_file = File::create(DB_FILE)?;
    Ok(db_file.write_all(&db_buf)?)
}
