
use conrod;
use conrod::image::Map;

use glium::texture::Texture2d;
use glium::backend::Facade;
use glium::Surface;

use window::Window;
use window::glium::GliumWindow;

use ui::UiManager;

pub struct OpenGLRenderer {
    ui_renderer: conrod::backend::glium::Renderer,
    image_map: Map<Texture2d>,
}

impl OpenGLRenderer {
    pub fn new<T: Facade>(window: &T) -> OpenGLRenderer {
        let ui_renderer = conrod::backend::glium::Renderer::new(window).expect("ui renderer creation error");
        let image_map: Map<Texture2d> = Map::new();

        OpenGLRenderer {
            ui_renderer,
            image_map,
        }
    }
}


pub trait Renderer {
    fn render(&mut self, window: &mut GliumWindow, ui: &mut UiManager);
}


impl Renderer for OpenGLRenderer {
    fn render(&mut self, window: &mut GliumWindow, ui: &mut UiManager) {
        let mut frame = window.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        if let Some(primitives) = ui.ui_mut().draw_if_changed() {
           self.ui_renderer.fill(window.display(), primitives, &self.image_map);
        }

        self.ui_renderer.draw(window, &mut frame, &self.image_map).expect("ui draw error");

        frame.finish().expect("error");
    }
}
