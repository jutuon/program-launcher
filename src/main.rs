
#[macro_use]
extern crate conrod;

extern crate glium;

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

fn main() {

    let mut window = GliumWindow::new("Program launcher", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
    let mut input = InputManager::new();
    let mut ui = ui::UiManager::new();

    let mut renderer = OpenGLRenderer::new(&window);

    let mut fps = FpsCounter::new();
    let mut time_manager = TimeManager::new();

    loop {
        time_manager.update_time(false);

        if window.update_input(&mut input, ui.ui_mut()) {
            ui.set_widgets();
        }

        if input.quit() {
            break;
        }

        renderer.render(&mut window, &mut ui);

        fps.frame();
        fps.update(time_manager.current_time(), true);
    }
}
