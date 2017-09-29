

use std::path::{Path, PathBuf};

use std::process::{Child, Command, Stdio};
use std::io::{Read};

use std::fmt::Write;

use std::sync::mpsc;
use std::thread;

use std::collections::VecDeque;

const MAX_LINES: usize = 100;

use Event;


pub struct TaskManager {
    queue: Vec<Command>,
    running_process: Option<Child>,
    console_lines: VecDeque<String>,
    byte_vec: Vec<u8>,
    stdout_receiver: Option<mpsc::Receiver<u8>>,
    stderr_receiver: Option<mpsc::Receiver<u8>>,
    library_directory: PathBuf,
}


impl TaskManager {
    pub fn new(library_directory: PathBuf) -> TaskManager {
        TaskManager {
            queue: vec![],
            console_lines: VecDeque::new(),
            running_process: None,
            byte_vec: vec![],
            stdout_receiver: None,
            stderr_receiver: None,
            library_directory,
        }
    }

    pub fn console_lines(&self) -> &VecDeque<String> {
        &self.console_lines
    }


    /// Return new console text if there is new text
    pub fn update<'a>(&'a mut self) -> Option<Event<'a>> {
        let mut process_finished = false;
        let mut stdout_or_stderr_update = false;

        if let Some(ref mut child) = self.running_process {
            if let Ok(Some(exit_status)) = child.try_wait() {
                // TODO: if exit status is non zero, clear command queue
                println!("{}", exit_status);
                process_finished = true;
            }
        }

        self.byte_vec.clear();

        if let Some(ref stdout) = self.stdout_receiver {
            for byte in stdout.try_iter() {
                self.byte_vec.push(byte);
                stdout_or_stderr_update = true;
            }
        }

        if let Some(ref stderr) = self.stderr_receiver {
            for byte in stderr.try_iter() {
                self.byte_vec.push(byte);
                stdout_or_stderr_update = true;
            }
        }

        if process_finished {
            self.running_process = None
        }

        let update_console =  if let Some(Event::ConsoleUpdate(_)) = self.pop_and_execute() {
            true
        } else {
            false
        };

        if stdout_or_stderr_update {
            for line in String::from_utf8_lossy(&self.byte_vec).lines() {
                self.console_lines.push_back(line.to_string());
            }

            while self.console_lines.len() > MAX_LINES {
                self.console_lines.pop_front();
            }
        }

        if update_console || stdout_or_stderr_update {
            Some(Event::ConsoleUpdate(&self.console_lines))
        } else {
            None
        }
    }

    /// Starts new process from the queue if there not currently exists an running process.
    pub fn pop_and_execute<'a>(&'a mut self) -> Option<Event<'a>> {
        if self.running_process.is_some() {
            return None;
        }

        if let Some(mut command) = self.queue.pop() {
            let mut text = String::new();
            writeln!(text, "\nStarted program: {:?}", command).unwrap();
            self.console_lines.push_back(text);

            match command.spawn() {
                Ok(mut child) => {
                    if let Some(stdout) = child.stdout {
                        child.stdout = None;

                        let (transmitter, receiver) = mpsc::channel();
                        self.stdout_receiver = Some(receiver);

                        // This thread should automatically close when process exits.
                        thread::spawn(move || {
                            read_and_send_process_output(stdout, transmitter);
                        });
                    }

                    if let Some(stderr) = child.stderr {
                        child.stderr = None;

                        let (transmitter, receiver) = mpsc::channel();
                        self.stderr_receiver = Some(receiver);

                        // This thread should automatically close when process exits.
                        thread::spawn(move || {
                            read_and_send_process_output(stderr, transmitter);
                        });
                    }

                    self.running_process = Some(child);
                }
                Err(error) => {
                    // TODO: clear command queue

                    println!("error: {}", error);

                    let mut text = String::new();
                    writeln!(text, "\nerror: {}", error).unwrap();
                    self.console_lines.push_back(text);
                }
            }
        }

        Some(Event::ConsoleUpdate(&self.console_lines))
    }

    /// Sets new commands to queue if there is not process running. If working directory is not found
    /// the download command will be first command in queue if there exists one.
    pub fn new_queue_if_no_running_process(&mut self, command_queue: &[CommandData], working_dir: &Path, download_command: &Option<CommandData>) {
        if self.running_process.is_some() {
            return;
        }

        self.queue.clear();

        if !working_dir.exists() {
            if let &Some(ref command_data) = download_command {
                self.queue.push(command_data.to_command(self.library_directory.as_path()));
            }
        }

        for data in command_queue {
            self.queue.push(data.to_command(working_dir));
        }

        self.queue.reverse();
    }
}

fn read_and_send_process_output<T: Read>(reader: T, transmitter: mpsc::Sender<u8>) {
    for byte_result in reader.bytes() {
        match byte_result {
            Ok(byte) => if let Err(error) = transmitter.send(byte) {
                println!("error when transmitting process output to main thread, {}", error);
                break;
            },
            Err(error) => {
                println!("error when reading process output: {}", error);
                break;
            }
        }
    }
}

use data::{CommandData};

impl CommandData {
    fn to_command(&self, working_dir: &Path) -> Command {
        let mut command = Command::new(&self.executable);
        command.args(&self.args).current_dir(working_dir).stdout(Stdio::piped()).stderr(Stdio::piped());
        command
    }
}
