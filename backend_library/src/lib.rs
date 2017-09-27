
#[macro_use]
extern crate serde_derive;

extern crate toml;

mod data;
mod task_manager;

use task_manager::TaskManager;


use data::{ProgramLibrary, Program};

use std::path::{Path, PathBuf};

pub struct ProgramLibraryManager {
    program_library: ProgramLibrary,
    library_directory_name: PathBuf,
    task_manager: TaskManager,
}

const LIBRARY_FILE_NAME: &'static str = "library.toml";

impl ProgramLibraryManager {
    pub fn new(library_directory_name: &str) -> Result<ProgramLibraryManager, Error> {
        let mut path = Path::new(library_directory_name).to_path_buf();

        data::create_library_directory_if_not_exists(path.as_path())?;

        path.push(LIBRARY_FILE_NAME);

        data::save_default_if_file_not_exists(path.as_path())?;
        let program_library = data::load_library(path.as_path())?;

        path.pop();

        let task_manager = TaskManager::new(path.as_path());

        let library_manager = ProgramLibraryManager {
            program_library,
            library_directory_name: path,
            task_manager,
        };

        Ok(library_manager)
    }


    pub fn programs(&self) -> &[Program] {
        &self.program_library.programs
    }

    pub fn start_program(&mut self, program_index: usize) {
        let program = &self.program_library.programs[program_index];

        self.library_directory_name.push(&program.directory_name);

        self.task_manager.push(TaskManager::cargo_run(self.library_directory_name.as_path()));

        self.library_directory_name.pop();
    }

    pub fn update_and_build_program(&mut self, program_index: usize) {
        let program = &self.program_library.programs[program_index];

        self.library_directory_name.push(&program.directory_name);

        if !self.library_directory_name.exists() {
            if let Some(path) = self.library_directory_name.parent() {
                self.task_manager.push(TaskManager::git_clone(&program.git_repository, &program.directory_name, path));
            } else {
                println!("directory parent directory not found from path");
                return;
            }
        }

        self.task_manager.push(TaskManager::cargo_build(self.library_directory_name.as_path()));
        self.task_manager.push(TaskManager::git_pull(self.library_directory_name.as_path()));

        self.library_directory_name.pop();
    }

    pub fn update(&mut self) -> Option<&str> {
        self.task_manager.update()
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError(toml::de::Error),
    IoError(std::io::Error),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
