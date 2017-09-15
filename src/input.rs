


pub struct InputManager {
    quit: bool,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            quit: false,
        }
    }
}

pub trait InputUpdater {
    fn set_quit(&mut self, value: bool);
}


pub trait Input {
    fn quit(&self) -> bool;
}

impl InputUpdater for InputManager {
    fn set_quit(&mut self, value: bool) {
        self.quit = value;
    }
}

impl Input for InputManager {
    fn quit(&self) -> bool { self.quit }
}
