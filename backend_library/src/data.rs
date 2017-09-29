

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::{File, create_dir};

use Error;
use serde_json;


#[derive(Deserialize)]
pub struct LibraryFileProgram {
    pub name: String,
    pub download_command: Option<CommandData>,
    pub working_directory: String,
    pub command_queues: Vec<CommandQueue>
}

#[derive(Deserialize)]
pub struct CommandQueue {
    pub name: String,
    pub commands: Vec<CommandData>,
}

#[derive(Deserialize)]
pub struct CommandData {
    pub executable: String,
    pub args: Vec<String>,
}


pub struct Program {
    pub name: String,
    pub download_command: Option<CommandData>,
    /// Absolute path
    pub working_directory: PathBuf,
    pub command_queues: Vec<CommandQueue>
}

pub struct ProgramLibrary {
    pub programs: Vec<Program>,
}



pub const DEFAULT_LIBRARY_FILE: &'static str = r#"

[
    {
        "name": "Space Boss Battles",
        "download_command": {
            "executable": "git",
            "args": ["clone", "https://github.com/jutuon/space-boss-battles", "space_boss_battles"]
        },
        "working_directory": "space_boss_battles",
        "command_queues" : [
            {
                "name": "Run",
                "commands": [
                    {
                        "executable" : "cargo",
                        "args": ["run", "--release"]
                    }
                ]
            },
            {
                "name": "Update and build",
                "commands": [
                    {
                        "executable" : "git",
                        "args": ["pull"]
                    },
                    {
                        "executable" : "cargo",
                        "args": ["build", "--release"]
                    }
                ]
            }
        ]
    }
]


"#;

pub(crate) fn create_library_directory_if_not_exists(directory_path: &Path) -> Result<(), Error> {
    if directory_path.exists() {
        Ok(())
    } else {
        match create_dir(directory_path) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub(crate) fn load_library(file_path: &Path, library_directory: &Path) -> Result<ProgramLibrary, Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(io_error) => return Err(Error::IoError(io_error)),
    };

    let mut text = String::new();

    match file.read_to_string(&mut text) {
        Ok(_) => (),
        Err(io_error) => return Err(Error::IoError(io_error)),
    }

    let library_file_programs: Vec<LibraryFileProgram> = match serde_json::from_str(&text) {
        Ok(library) => library,
        Err(parse_error) => return Err(Error::ParseError(parse_error)),
    };

    let programs = library_file_programs.into_iter().map(|item| {
        let mut working_directory = library_directory.to_path_buf();
        // push replaces current path if argument is absolute path
        working_directory.push(item.working_directory);

        Program {
            name: item.name,
            download_command: item.download_command,
            working_directory,
            command_queues: item.command_queues,
        }
    }).collect();

    Ok(ProgramLibrary {
        programs
    })
}

pub(crate) fn save_default_if_file_not_exists(file_path: &Path, default_file_contents: &str) -> Result<(), Error> {
    if file_path.exists() {
        return Ok(())
    } else {
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(io_error) => return Err(Error::IoError(io_error)),
        };

        match file.write_all(default_file_contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub(crate) fn create_empty_file_if_not_exists(file_path: &Path) -> Result<(), Error> {
    if file_path.exists() {
        Ok(())
    } else {
        match File::create(file_path) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::IoError(io_error)),
        }
    }
}

pub(crate) fn write_empty_file(file_path: &Path) -> Result<(), Error> {
    match File::create(file_path) {
        Ok(_) => Ok(()),
        Err(io_error) => Err(Error::IoError(io_error)),
    }
}