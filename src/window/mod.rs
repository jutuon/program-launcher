

pub mod sdl2;
pub mod glium;

use conrod::Ui;
use input::InputUpdater;

use glium::Frame;
use glium::backend::Facade;

pub trait Window : Facade {
    fn new(title: &str, width: u32, height: u32) -> Self;

    /// Returns true if there were event for ui.
    fn update_input<T: InputUpdater>(&mut self, update: &mut T, ui: &mut Ui) -> bool;
    fn draw(&mut self) -> Frame;
}