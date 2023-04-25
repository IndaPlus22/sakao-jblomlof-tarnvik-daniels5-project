use std::path::Path;

use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use graphics::{
    types::{Color, Matrix2d, Vec2d},
    DrawState, Image, Rectangle,
};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

pub struct Button {
    dims: [f64; 4],
    shape: Rectangle,
    pub hover: bool,
    texture: Texture,
}

impl Button {
    pub fn new(pos: Vec2d, width: f64, height: f64, color: Color, img_path: &str) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let shape = Rectangle::new(color);
        let texture = Texture::from_path(img_path, &TextureSettings::new()).unwrap();
        // let img = image::open(tmp_path).unwrap();
        // let image_rgba = img.to_rgba8();

        // //A texture to use with the image
        // let texture = Texture::from_image(&image_rgba, &TextureSettings::new());
        Button {
            dims,
            shape,
            hover: false,
            texture,
        }
    }

    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        let img = Image::new().rect(self.dims);
        img.draw(&self.texture, &DrawState::default(), transform, gl);
        // self.shape.draw(
        //     self.dims,
        //     &DrawState::default(),
        //     transform,
        //     gl,
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
