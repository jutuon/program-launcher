
#[macro_use]
extern crate conrod;

extern crate glium;
extern crate backend_library;

pub mod input;
pub mod window;
pub mod ui;
pub mod renderer;
pub mod utils;

use renderer::{Renderer, OpenGLRenderer};

use window::glium::GliumWindow;
use window::Window;
use input::{Input, InputManager};

use utils::{FpsCounter, TimeManager};

const DEFAULT_WINDOW_WIDTH: u32 = 640;
const DEFAULT_WINDOW_HEIGHT: u32 = 480;

const LIBRARY_DIRECTORY_NAME: &'static str = "program_launcher_library";

use backend_library::{ProgramLibraryManager, Event};



fn main() {

    let mut full_screen = false;

    for arg in std::env::args().skip(1) {
        if arg == "--fullscreen" {
            full_screen = true;
        }
    }

    let mut library = ProgramLibraryManager::new(LIBRARY_DIRECTORY_NAME).expect("library loading error");


    let mut window = GliumWindow::new("Program launcher", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT, full_screen);
    let mut input = InputManager::new();
    let mut ui = ui::UiManager::new();

    let mut renderer = OpenGLRenderer::new(&window);

    let mut fps = FpsCounter::new();
    let mut time_manager = TimeManager::new();

    loop {
        time_manager.update_time(false);

        let mut console_text_update = false;
        {
            if let Some(Event::ConsoleUpdate(console_lines)) = library.update() {
                console_text_update = true;
                ui.update_console_text(console_lines);
            }
        }

        if window.update_input(&mut input, ui.ui_mut()) || console_text_update {
            let (task_manager, programs) = library.task_manager_mut_and_programs();
            ui.set_widgets(task_manager, programs, &mut window);
        }

        if input.quit() {
            break;
        }

        //std::thread::sleep(std::time::Duration::from_millis(16));

        renderer.render(&mut window, &mut ui);

        fps.frame();

        fps.update(time_manager.current_time(), true);


    }
}
