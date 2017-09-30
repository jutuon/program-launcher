

use window::Window;

use input::InputUpdater;

use conrod::Ui;
use conrod::backend::winit::{convert_event};
use glium::backend::{Facade, Context};
use glium::{Display, Frame};

use std::rc::Rc;


use utils::TimeMilliseconds;

use glium::glutin::{
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

use glium::glutin;

pub struct GliumWindow {
    display: Display,
    event_loop: EventsLoop,
    full_screen: bool,
}

impl GliumWindow {
    pub fn display(&self) -> &Display {
        &self.display
    }
}

impl Window for GliumWindow {
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

        GliumWindow {
            display,
            event_loop,
            full_screen,
        }
    }

    fn update_input<T: InputUpdater>(&mut self, update: &mut T, ui: &mut Ui, current_time: &TimeMilliseconds) -> bool {
        let display = &self.display;
        let mut event_update = false;

        self.event_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::Closed, ..} => {
                    update.set_quit(true);
                    event_update = true;
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput {input, ..}, ..} => {
                    if handle_keyboard_input(update, input, current_time) {
                        event_update = true;
                    }
                }
                //event => println!("{:?}", event),
                _ => (),
            };

            if let Some(conrod_event) = convert_event(event, display) {
                ui.handle_event(conrod_event);
                event_update = true;
            }
        });

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

impl Facade for GliumWindow {
    fn get_context(&self) -> &Rc<Context> {
        self.display.get_context()
    }
}

/// Returns true if InputUpdater was updated.
fn handle_keyboard_input<T: InputUpdater>(update: &mut T, keyboard_input: KeyboardInput, current_time: &TimeMilliseconds) -> bool {
    use input::utils::KeyEvent;
    use glium::glutin::ElementState;

    let mut updated_input = true;

    match keyboard_input {
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Escape), ..} => update.set_quit(true),
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



