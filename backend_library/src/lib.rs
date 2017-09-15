
#[macro_use]
extern crate serde_derive;

extern crate toml;

mod data;

use data::{ProgramLibrary, Program};

pub struct ProgramLibraryManager {
    program_library: ProgramLibrary,
}

impl ProgramLibraryManager {
    pub fn new(library_file_name: &str) -> Result<ProgramLibraryManager, Error> {
        data::save_default_if_file_not_exists(library_file_name)?;

        let program_library = data::load_library(library_file_name)?;

        let library_manager = ProgramLibraryManager {
            program_library
        };

        Ok(library_manager)
    }


    pub fn programs(&self) -> &[Program] {
        &self.program_library.programs
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
