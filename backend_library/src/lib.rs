
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

pub mod data;
pub mod task_manager;

use task_manager::TaskManager;


use data::{ProgramLibrary};

use std::path::{Path, PathBuf};

pub struct ProgramLibraryManager {
    program_library: ProgramLibrary,
    task_manager: TaskManager,
}

const LIBRARY_FILE_NAME: &'static str = "library.json";

impl ProgramLibraryManager {
    pub fn new(library_directory_name: &str) -> Result<ProgramLibraryManager, Error> {
        // Handle library directory creation
        let library_directory = Path::new(library_directory_name).to_path_buf();
        data::create_library_directory_if_not_exists(library_directory.as_path())?;

        // Handle library file creation and loading
        let mut library_file_path = library_directory.clone();
        library_file_path.push(LIBRARY_FILE_NAME);

        data::save_default_if_file_not_exists(library_file_path.as_path(), data::DEFAULT_LIBRARY_FILE)?;
        let program_library = data::load_library(library_file_path.as_path(), library_directory.as_path())?;

        let task_manager = TaskManager::new(library_directory);

        let library_manager = ProgramLibraryManager {
            program_library,
            task_manager,
        };

        Ok(library_manager)
    }


    pub fn task_manager_mut_and_programs(&mut self) -> (&mut TaskManager,  &ProgramLibrary) {
        (&mut self.task_manager, &self.program_library)
    }

    pub fn update(&mut self) -> Option<&str> {
        self.task_manager.update()
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(serde_json::error::Error),
    IoError(std::io::Error),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
