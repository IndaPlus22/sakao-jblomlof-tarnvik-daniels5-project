use graphics::{
    types::{Matrix2d, Vec2d},
    DrawState, Image,
};
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

pub struct Button {
    dims: [f64; 4],
    hover: bool,
    texture: Texture,
}

impl Button {
    pub fn new(pos: Vec2d, width: f64, height: f64, img_path: &str) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let texture = Texture::from_path(img_path, &TextureSettings::new()).unwrap();

        Button {
            dims,
            hover: false,
            texture,
        }
    }

    pub fn init_buttons(button_size: Vec2d, position: Vec2d, sprites: [&str; 5], distance: Vec2d) -> Vec<Button> {
        let mut buttons = Vec::new();

        for i in 0..5 {
            let button = Button::new(
                [position[0] + distance[0]*(button_size[1] * i as f64), 
                position[1] + distance[1]*(button_size[1] * i as f64)],
                button_size[0],
                button_size[1],
                &sprites[i],
            );
            buttons.push(button);
        }
        buttons
    }

    pub fn get_hover (&self) -> bool{
        self.hover
    }

    pub fn draw(&self, gl: &mut GlGraphics, transform: Matrix2d) {
        let img = Image::new().rect(self.dims);
        img.draw(&self.texture, &DrawState::default(), transform, gl);
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
