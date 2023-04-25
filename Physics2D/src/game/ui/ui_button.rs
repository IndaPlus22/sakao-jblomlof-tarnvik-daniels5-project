use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::{
    types::{Color, Matrix2d, Vec2d},
    Rectangle,
};
use opengl_graphics::{Texture, TextureSettings};
use graphics::{Image};

pub struct Button {
    dims: [f64; 4],
    shape: Rectangle,
    pub hover: bool,
    texture: Texture,
}

impl Button {
    pub fn new(pos: Vec2d, width: f64, height: f64, color: Color) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let shape = Rectangle::new(color);

        // not working
        let tmp_path = "sprites/ui/tool_bar/test.png";
        let texture = Button::load_sprite(tmp_path);
        Button {
            dims,
            shape,
            hover: false,
            texture,
        }
    }

    fn load_sprite(path: &str) -> Texture {
        let texture = Texture::from_path(path, &TextureSettings::new()).unwrap();
        texture
    }

    pub fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        gl.draw(
            &self.dims,
            &piston_window::DrawState::default(),
            transform,
            graphics,
        );
        // self.shape.draw(
        //     self.dims,
        //     &piston_window::DrawState::default(),
        //     transform,
        //     graphics,
        // );
    }

    pub fn check_hover(&mut self, mouse_position: Vec2d) {
        if mouse_position[0] > self.dims[0]
            && mouse_position[0] < self.dims[0] + self.dims[2]
            && mouse_position[1] > self.dims[1]
            && mouse_position[1] < self.dims[1] + self.dims[3]
        {
            self.hover = true;
        } else {
            self.hover = false;
        }
    }
}
