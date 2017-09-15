

use window::Window;

use input::InputUpdater;

use conrod::Ui;
use conrod::backend::winit::{convert_event};
use glium::backend::{Facade, Context};
use glium::{Display, Frame};

use std::rc::Rc;


use glium::glutin::{
    WindowBuilder,
    Event,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
    EventsLoop,
    ContextBuilder,
    GlRequest,
    Api
};

pub struct GliumWindow {
    display: Display,
    event_loop: EventsLoop,
}

impl GliumWindow {
    pub fn display(&self) -> &Display {
        &self.display
    }
}

impl Window for GliumWindow {
    fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);

        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (2,0)));

        let display = Display::new(window, context, &event_loop).expect("error");

        GliumWindow {
            display,
            event_loop,
        }
    }

    fn update_input<T: InputUpdater>(&mut self, update: &mut T, ui: &mut Ui) -> bool {
        let display = &self.display;
        let mut ui_event = false;

        self.event_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::Closed, ..} => update.set_quit(true),
                Event::WindowEvent { event: WindowEvent::KeyboardInput {input, ..}, ..} => {
                    handle_keyboard_input(update, input);
                }
                //event => println!("{:?}", event),
                _ => (),
            };

            if let Some(conrod_event) = convert_event(event, display) {
                ui.handle_event(conrod_event);
                ui_event = true;
            }
        });

        return ui_event;
    }

    fn draw(&mut self) -> Frame {
        self.display.draw()
    }
}

impl Facade for GliumWindow {
    fn get_context(&self) -> &Rc<Context> {
        self.display.get_context()
    }
}

fn handle_keyboard_input<T: InputUpdater>(update: &mut T, keyboard_input: KeyboardInput) {
    match keyboard_input {
        KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Escape), ..} => update.set_quit(true),
        _ => (),
    }
}



