
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::glutin;
use gilrs;

use conrod::Ui;
use conrod::backend::winit::{convert_event};

use self::glium::backend::{Facade, Context};
use self::glium::{Display, Frame};

use gilrs::{Gilrs};

use std::rc::Rc;

use window::Window;
use input::InputUpdater;
use utils::TimeMilliseconds;

use self::glutin::{
    WindowBuilder,
    Event,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
    EventsLoop,
    ContextBuilder,
    GlRequest,
    Api,
};


pub struct GlutinWindow {
    display: Display,
    event_loop: EventsLoop,
    full_screen: bool,
    window_focused: bool,
    game_controllers: Gilrs,
}

impl GlutinWindow {
    pub fn display(&self) -> &Display {
        &self.display
    }
}

impl Window for GlutinWindow {
    fn new(title: &str, width: u32, height: u32, full_screen: bool) -> Self {
        let event_loop = EventsLoop::new();

        let mut window = WindowBuilder::new()
            .with_title(title);

        if full_screen {
            window = window.with_fullscreen(glutin::get_primary_monitor());
        } else {
            window = window.with_dimensions(width, height);
        }

        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (2,0)));

        let display = Display::new(window, context, &event_loop).expect("error");

        GlutinWindow {
            display,
            event_loop,
            full_screen,
            window_focused: true,
            game_controllers: Gilrs::new(),
        }
    }

    fn update_input<T: InputUpdater>(&mut self, update: &mut T, ui: &mut Ui, current_time: &TimeMilliseconds) -> bool {
        let display = &self.display;
        let mut event_update = false;
        let mut window_focused = self.window_focused;

        self.event_loop.poll_events(|event| {
            match event.clone() {
                Event::WindowEvent { event: WindowEvent::Closed, ..} => {
                    update.set_quit(true);
                    event_update = true;
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput {input, ..}, ..} => {
                    if handle_keyboard_input(update, input, current_time) {
                        event_update = true;
                    }
                }
                Event::WindowEvent { event: WindowEvent::Focused(value), ..} => window_focused = value,
                //event => println!("{:?}", event),
                _ => (),
            };

            if let Some(conrod_event) = convert_event(event, display) {
                ui.handle_event(conrod_event);
                event_update = true;
            }
        });

        self.window_focused = window_focused;

        for (_, e) in self.game_controllers.poll_events() {
            if self.window_focused && handle_game_controller_input(update, e, current_time) {
                event_update = true;
            }
        }

        return event_update;
    }

    fn draw(&mut self) -> Frame {
        self.display.draw()
    }

    fn full_screen(&self) -> bool {
        self.full_screen
    }

    fn set_full_screen(&mut self, value: bool) {
        self.full_screen = value;
        // TODO: full screen support
    }
}

impl Facade for GlutinWindow {
    fn get_context(&self) -> &Rc<Context> {
        self.display.get_context()
    }
}

/// Returns true if InputUpdater was updated.
fn handle_keyboard_input<T: InputUpdater>(update: &mut T, keyboard_input: KeyboardInput, current_time: &TimeMilliseconds) -> bool {
    use input::utils::KeyEvent;
    use self::glutin::ElementState;

    let mut updated_input = true;

    match keyboard_input {
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Escape), ..} => update.set_quit(true),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Return), state: ElementState::Released, ..} => update.set_select(true),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Left), state: ElementState::Pressed, ..} => update.update_left(KeyEvent::KeyDown, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Left), state: ElementState::Released, ..} => update.update_left(KeyEvent::KeyUp, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Right), state: ElementState::Pressed, ..} => update.update_right(KeyEvent::KeyDown, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Right), state: ElementState::Released, ..} => update.update_right(KeyEvent::KeyUp, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Up), state: ElementState::Pressed, ..} => update.update_up(KeyEvent::KeyDown, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Up), state: ElementState::Released, ..} => update.update_up(KeyEvent::KeyUp, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Down), state: ElementState::Pressed, ..} => update.update_down(KeyEvent::KeyDown, current_time),
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Down), state: ElementState::Released, ..} => update.update_down(KeyEvent::KeyUp, current_time),
        _ => updated_input = false,
    }

    updated_input
}

/// Returns true if InputUpdater was updated.
fn handle_game_controller_input<T: InputUpdater>(update: &mut T, event: gilrs::Event, current_time: &TimeMilliseconds) -> bool {
    let mut updated_input = true;

    use gilrs::Event::{ButtonReleased, ButtonPressed, AxisChanged};
    use gilrs::Button::{South, DPadUp, DPadDown, DPadLeft, DPadRight};

    use gilrs::Axis::{LeftStickX, LeftStickY};

    use input::utils::KeyEvent::{KeyDown, KeyUp};

    match event {
        ButtonReleased(South, _)     => update.set_select(true),

        ButtonPressed(DPadLeft, _)  => update.update_left(KeyDown, current_time),
        ButtonPressed(DPadRight, _) => update.update_right(KeyDown, current_time),
        ButtonPressed(DPadUp,_)     => update.update_up(KeyDown, current_time),
        ButtonPressed(DPadDown,_)   => update.update_down(KeyDown, current_time),

        ButtonReleased(DPadLeft, _)   => update.update_left(KeyUp, current_time),
        ButtonReleased(DPadRight, _)  => update.update_right(KeyUp, current_time),
        ButtonReleased(DPadUp, _)     => update.update_up(KeyUp, current_time),
        ButtonReleased(DPadDown, _)   => update.update_down(KeyUp, current_time),

        AxisChanged(LeftStickX, value, _) => {
            let limit = 0.3;
            if value >= limit {
                update.update_right(KeyDown, current_time);
            } else if value <= -limit {
                update.update_left(KeyDown, current_time);
            } else {
                update.update_left(KeyUp, current_time);
                update.update_right(KeyUp, current_time);
            }
        }

        AxisChanged(LeftStickY, value, _) => {
            let limit = 0.3;
            if value >= limit {
                update.update_up(KeyDown, current_time);
            } else if value <= -limit {
                update.update_down(KeyDown, current_time);
            } else {
                update.update_up(KeyUp, current_time);
                update.update_down(KeyUp, current_time);
            }
        }
        _ => updated_input = false,
    }

    updated_input
}

