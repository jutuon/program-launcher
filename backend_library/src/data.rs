

use std::io::{Read, Write};
use std::fs::{File};
use std::path::Path;

use Error;
use toml;

#[derive(Deserialize)]
pub enum BuildSystem {
    Cargo,
}

#[derive(Deserialize)]
pub struct Program {
    pub name: String,
    pub git_repository: String,
    pub build_system: BuildSystem,
}

#[derive(Deserialize)]
pub struct ProgramLibrary {
    pub programs: Vec<Program>,
}


const DEFAULT_LIBRARY_FILE: &'static str = "

# Program launcher library file

[[programs]]
name = \"Space Boss Battles\"
git_repository = \"https://github.com/jutuon/space-boss-battles\"
build_system = \"Cargo\"


";


pub fn load_library(file_name: &str) -> Result<ProgramLibrary, Error> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(io_error) => return Err(Error::IoError(io_error)),
    };

    let mut text = String::new();

    match file.read_to_string(&mut text) {
        Ok(_) => (),
        Err(io_error) => return Err(Error::IoError(io_error)),
    }

    match toml::from_str(&text) {
        Ok(library) => Ok(library),
        Err(parse_error) => Err(Error::ParseError(parse_error)),
    }
}

pub fn save_default_if_file_not_exists(file_name: &str) -> Result<(), Error> {
    let path = Path::new(file_name);

    if path.exists() {
        return Ok(());
    }

    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(io_error) => return Err(Error::IoError(io_error)),
    };

    match file.write_all(DEFAULT_LIBRARY_FILE.as_bytes()) {
        Ok(_) => Ok(()),
        Err(io_error) => Err(Error::IoError(io_error)),
    }
}