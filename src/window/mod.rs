

#[cfg(feature = "sdl2-window")]
pub mod sdl2;

#[cfg(feature = "glutin-window")]
pub mod glutin;

use conrod::Ui;
use input::InputUpdater;

use conrod::backend::glium::glium::Frame;
use conrod::backend::glium::glium::backend::Facade;

use utils::TimeMilliseconds;

pub trait Window : Facade {
    fn new(title: &str, width: u32, height: u32, full_screen: bool) -> Self;

    /// Returns true if ui or InputUpdater was updated.
    fn update_input<T: InputUpdater>(&mut self, update: &mut T, ui: &mut Ui, current_time: &TimeMilliseconds) -> bool;
    fn draw(&mut self) -> Frame;

    fn full_screen(&self) -> bool;

    fn set_full_screen(&mut self, value: bool);
}