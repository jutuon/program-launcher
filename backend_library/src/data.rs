

use std::io::{Read, Write};
use std::fs::{File, create_dir};
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
    pub directory_name: String,
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
directory_name = \"space_boss_battles\"
build_system = \"Cargo\"


";

pub fn create_library_directory_if_not_exists(directory_path: &Path) -> Result<(), Error> {
    if directory_path.exists() {
        Ok(())
    } else {
        match create_dir(directory_path) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub fn load_library(file_path: &Path) -> Result<ProgramLibrary, Error> {
    let mut file = match File::open(file_path) {
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

pub fn save_default_if_file_not_exists(file_path: &Path) -> Result<(), Error> {
    if file_path.exists() {
        return Ok(())
    } else {
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(io_error) => return Err(Error::IoError(io_error)),
        };

        match file.write_all(DEFAULT_LIBRARY_FILE.as_bytes()) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub fn create_empty_file_if_not_exists(file_path: &Path) -> Result<(), Error> {
    if file_path.exists() {
        Ok(())
    } else {
        match File::create(file_path) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub fn write_empty_file(file_path: &Path) -> Result<(), Error> {
    match File::create(file_path) {
        Ok(_) => Ok(()),
        Err(io_error) => Err(Error::IoError(io_error)),
    }
}