include!(concat!(env!("OUT_DIR"), "/ongaku.db.rs"));

use std::{fs, io::Write};

use prost::Message;

use crate::error::OngakuError;

static DB_FILE: &str = ".ongaku.pb";
static DB_VERSION: u32 = 1;

pub fn init() -> Result<(), OngakuError> {
    let library = Library {
        version: DB_VERSION,
        entry: Vec::new(),
    };

    let mut db_file = fs::File::create_new(DB_FILE).map_err(|_| OngakuError::AlreadyInitialized)?;

    let mut db_buf: Vec<u8> = Vec::new();
    library
        .encode(&mut db_buf)
        .map_err(|e| OngakuError::DbWriteFailed(e.to_string()))?;
    db_file
        .write_all(&db_buf)
        .map_err(|e| OngakuError::DbWriteFailed(e.to_string()))?;

    Ok(())
}

