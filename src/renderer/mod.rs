
use conrod::image::Map;

use conrod::backend::glium::glium::texture::Texture2d;
use conrod::backend::glium::glium::backend::Facade;
use conrod::backend::glium::glium::Surface;

use conrod::backend::glium::Renderer as UiRenderer;

use window::Window;

use ui::UiManager;

pub struct OpenGLRenderer {
    ui_renderer: UiRenderer,
    image_map: Map<Texture2d>,
}

impl OpenGLRenderer {
    pub fn new<T: Facade>(window: &T) -> OpenGLRenderer {
        let ui_renderer = UiRenderer::new(window).expect("ui renderer creation error");
        let image_map: Map<Texture2d> = Map::new();

        OpenGLRenderer {
            ui_renderer,
            image_map,
        }
    }
}


pub trait Renderer {
    fn render<T: Window>(&mut self, window: &mut T, ui: &mut UiManager);
}


impl Renderer for OpenGLRenderer {
    fn render<T: Window>(&mut self, window: &mut T, ui: &mut UiManager) {
        let mut frame = window.draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);

        if let Some(primitives) = ui.ui_mut().draw_if_changed() {
           self.ui_renderer.fill_non_glutin_window(window.opengl_version(), window.width_and_height(), window.dpi_factor(), primitives, &self.image_map);
        }

        self.ui_renderer.draw(window, &mut frame, &self.image_map).expect("ui draw error");

        frame.finish().expect("error");
    }
}
