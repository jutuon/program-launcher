
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

use backend_library::{ProgramLibraryManager};



fn main() {

    let mut library = ProgramLibraryManager::new(LIBRARY_DIRECTORY_NAME).expect("library loading error");


    let mut window = GliumWindow::new("Program launcher", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
    let mut input = InputManager::new();
    let mut ui = ui::UiManager::new();

    let mut renderer = OpenGLRenderer::new(&window);

    let mut fps = FpsCounter::new();
    let mut time_manager = TimeManager::new();

    loop {
        time_manager.update_time(false);

        let mut console_text_update = false;
        {

            //let mut new_stdout = library.update();
            if let Some(text) = library.update() {
                ui.update_console_text(text);
                console_text_update = true;
            }

        }

        if window.update_input(&mut input, ui.ui_mut()) || console_text_update {
            let (task_manager, programs) = library.task_manager_mut_and_programs();
            ui.set_widgets(task_manager, programs);
        }

        if input.quit() {
            break;
        }

        renderer.render(&mut window, &mut ui);

        fps.frame();
        fps.update(time_manager.current_time(), true);


    }
}
