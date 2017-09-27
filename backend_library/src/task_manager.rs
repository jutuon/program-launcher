

use std::path::{Path, PathBuf};

use std::process::{Child, Command, Stdio};
use std::io::{Read, Stdin};
use std::fs::{OpenOptions, File};

use Error;

use data::{create_empty_file_if_not_exists, write_empty_file};

const STDOUT_LOG_FILE: &'static str = "stdout_log.txt";
const STDERR_LOG_FILE: &'static str = "stderr_log.txt";

pub struct TaskManager {
    tasks: Vec<Command>,
    running_process: Option<Child>,
    stdout_and_stderr_string: String,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
    stdout_file: File,
    stderr_file: File,
}


impl TaskManager {
    pub fn new(library_folder_path: &Path) -> TaskManager {

        let mut stdout_path = library_folder_path.to_path_buf();
        stdout_path.push(STDOUT_LOG_FILE);
        write_empty_file(stdout_path.as_path()).expect("could not create stdout log file");

        let mut stderr_path = library_folder_path.to_path_buf();
        stderr_path.push(STDERR_LOG_FILE);
        write_empty_file(stderr_path.as_path()).expect("could not create stderr log file");

        let stdout_file = File::open(stdout_path.as_path()).expect("could not open stdout log file");
        let stderr_file = File::open(stderr_path.as_path()).expect("could not open stderr log file");

        TaskManager {
            tasks: vec!(),
            stdout_and_stderr_string: String::new(),
            running_process: None,
            stdout_path,
            stderr_path,
            stdout_file,
            stderr_file,
        }
    }

    pub fn update(&mut self) -> Option<&str> {
        let mut process_finished = false;
        let mut stdout_or_stderr_update = false;

        if let Some(ref mut child) = self.running_process {
            match self.stdout_file.read_to_string(&mut self.stdout_and_stderr_string) {
                Ok(byte_count) => {
                    if byte_count > 0 {
                        stdout_or_stderr_update = true;
                    }
                },
                Err(error) => panic!("could not read from stdout log: {}", error),
            }

            match self.stderr_file.read_to_string(&mut self.stdout_and_stderr_string) {
                Ok(byte_count) => {
                    if byte_count > 0 {
                        stdout_or_stderr_update = true;
                    }
                },
                Err(error) => panic!("could not read from stderr log: {}", error),
            }

            if let Ok(Some(exit_status)) = child.try_wait() {
                println!("{}", exit_status);
                process_finished = true;
            }
        }

        if process_finished {
            self.running_process = None
        }

        self.pop_and_execute();

        if stdout_or_stderr_update {
            Some(&self.stdout_and_stderr_string)
        } else {
            None
        }
    }

    pub fn push(&mut self, mut command: Command) {
        if self.running_process.is_none() {
            let stdout = OpenOptions::new().append(true).open(self.stdout_path.as_path()).expect("stdout log file append mode failed");
            let stderr = OpenOptions::new().append(true).open(self.stderr_path.as_path()).expect("stderr log file append mode failed");

            command.stdout(stdout);
            command.stderr(stderr);

            self.tasks.push(command);
        }
    }

    pub fn pop_and_execute(&mut self) {
        if self.running_process.is_some() {
            return;
        }

        if let Some(mut command) = self.tasks.pop() {
            match command.spawn() {
                Ok(child) => self.running_process = Some(child),
                Err(error) => println!("error: {}", error),
            }
        }
    }

    pub fn git_clone(repository: &str, directory_name: &str, working_dir: &Path) -> Command {
        let mut command = Command::new("git");
        command.args(&["clone", repository, directory_name]).current_dir(working_dir);

        command
    }

    pub fn git_pull(working_dir: &Path) -> Command {
        let mut command = Command::new("git");
        command.arg("pull").current_dir(working_dir);

        command
    }

    pub fn cargo_run(working_dir: &Path) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["run", "--release"]).current_dir(working_dir);

        command
    }

    pub fn cargo_build(working_dir: &Path) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["build", "--release"]).current_dir(working_dir);
        command
    }


}
