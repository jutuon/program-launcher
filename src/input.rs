

use input::utils::{KeyHitGenerator, KeyEvent};

use utils::TimeMilliseconds;

pub struct InputManager {
    quit: bool,
    right: KeyHitGenerator,
    left: KeyHitGenerator,
    up: KeyHitGenerator,
    down: KeyHitGenerator,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            quit: false,
            right: KeyHitGenerator::new(),
            left: KeyHitGenerator::new(),
            up: KeyHitGenerator::new(),
            down: KeyHitGenerator::new(),
        }
    }

    pub fn update(&mut self, current_time: &TimeMilliseconds) {
        self.up.update(current_time, true);
        self.down.update(current_time, true);
        self.left.update(current_time, true);
        self.right.update(current_time, true);
    }
}

pub trait InputUpdater {
    fn set_quit(&mut self, value: bool);
    fn update_right(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds);
    fn update_left(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds);
    fn update_up(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds);
    fn update_down(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds);
}


pub trait Input {
    fn quit(&self) -> bool;
    fn right(&mut self) -> bool;
    fn left(&mut self) -> bool;
    fn up(&mut self) -> bool;
    fn down(&mut self) -> bool;
}

impl InputUpdater for InputManager {
    fn set_quit(&mut self, value: bool) {
        self.quit = value;
    }

    fn update_right(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds) { self.right.update_from_key_event(key_event, current_time) }
    fn update_left(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds) { self.left.update_from_key_event(key_event, current_time) }
    fn update_up(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds) { self.up.update_from_key_event(key_event, current_time) }
    fn update_down(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds) { self.down.update_from_key_event(key_event, current_time) }
}

impl Input for InputManager {
    fn quit(&self) -> bool { self.quit }
    fn right(&mut self) -> bool { self.right.key_hit() }
    fn left(&mut self) -> bool { self.left.key_hit() }
    fn up(&mut self) -> bool { self.up.key_hit() }
    fn down(&mut self) -> bool { self.down.key_hit() }
}




pub mod utils {

    //! Utilities for `input` module's objects.

    use utils::{Timer, TimeMilliseconds};

    /// Key press states.
    #[derive(Clone)]
    pub enum KeyEvent {
        KeyUp,
        KeyDown,
    }

    /// KeyHitGenerator's states.
    enum KeyHitState {
        /// Normal key hits.
        NormalMode,
        /// Generator generated key hits.
        ScrollMode,
    }

    /// Generate key hits.
    ///
    /// Generates key hits from key up event and if the key is pressed down
    /// long enough, the generator will generate multiple key hits.
    pub struct KeyHitGenerator {
        milliseconds_between_key_hits: u32,
        timer: Timer,
        state: Option<KeyHitState>,
        key_hit: bool,
    }

    impl KeyHitGenerator {
        /// Create new `KeyHitGenerator` which `milliseconds_between_key_hits` field is set to `300`.
        pub fn new() -> KeyHitGenerator {
            KeyHitGenerator {
                milliseconds_between_key_hits: 300,
                timer: Timer::new(),
                state: None,
                key_hit: false,
            }
        }

        /// Updates generators state from `KeyEvent`.
        pub fn update_from_key_event(&mut self, key_event: KeyEvent, current_time: &TimeMilliseconds) {
            match key_event {
                KeyEvent::KeyUp => self.up(),
                KeyEvent::KeyDown => self.down(current_time),
            }
        }

        /// Update method will generate key hits if
        ///
        /// * There is enough time passed from the last key hit.
        /// * `key_down` argument is true.
        pub fn update(&mut self, current_time: &TimeMilliseconds, key_down: bool) {
            // TODO: this can be removed?
            if !key_down {
                return;
            }

            match self.state {
                Some(KeyHitState::NormalMode) => {
                    if self.timer.check(current_time, self.milliseconds_between_key_hits) {
                        self.state = Some(KeyHitState::ScrollMode);
                        self.key_hit = true;
                    }
                },
                Some(KeyHitState::ScrollMode) => {
                    if self.timer.check(current_time, self.milliseconds_between_key_hits) {
                        self.key_hit = true;
                    }
                },
                _ => (),
            }
        }

        /// Handle key down event.
        ///
        /// Sets generators state to `Some(KeyHitState::NormalMode)` and resets
        /// generators internal timer if generators current state is `None`.
        fn down(&mut self, current_time: &TimeMilliseconds) {
            match self.state {
                None => {
                    self.state = Some(KeyHitState::NormalMode);
                    self.timer.reset(current_time);
                },
                _ => (),
            }
        }

        /// Handle key up event.
        ///
        /// Creates a key hit if method is called when
        /// generators state is `Some(KeyHitState::NormalMode)`.
        ///
        /// Generators state will be set to `None`.
        fn up(&mut self) {
            if let Some(KeyHitState::NormalMode) = self.state {
                self.key_hit = true;
            } else {
                self.key_hit = false;
            };

            self.state = None;
        }

        /// Returns true if key hit has been happened.
        ///
        /// This method will also clear the current key hit.
        pub fn key_hit(&mut self) -> bool {
            if self.key_hit {
                self.clear();
                true
            } else {
                false
            }
        }

        /// Clears current key hit.
        pub fn clear(&mut self) {
            self.key_hit = false;
        }
    }
}