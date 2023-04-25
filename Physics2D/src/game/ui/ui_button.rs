use std::path::Path;

use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use opengl_graphics::{Texture, TextureSettings, GlGraphics};
use graphics::{Image, types::{Vec2d, Color, Matrix2d}, Rectangle, DrawState};

pub struct Button {
    dims: [f64; 4],
    shape: Rectangle,
    pub hover: bool,
    // texture: Texture,
}

impl Button {
    pub fn new(pos: Vec2d, width: f64, height: f64, color: Color) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let shape = Rectangle::new(color);

        // not working
        // let tmp_path = "sprites/ui/tool_bar/test.png";
        // let texture = Button::load_sprite(tmp_path);
        Button {
            dims,
            shape,
            hover: false,
            // texture,
        }
    }

    fn load_sprite(path: &str) -> Texture {
        let texture = Texture::from_path(path, &TextureSettings::new()).unwrap();
        texture
    }

    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        // let img = Image::new().rect(self.dims);
        // img.draw(
        //     &self.texture,
        //     &DrawState::default(),
        //     transform,
        //     gl,
        // );
        self.shape.draw(
            self.dims,
            &DrawState::default(),
            transform,
            gl,
        );
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
