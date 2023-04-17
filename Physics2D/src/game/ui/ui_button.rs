use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use piston_window::{Rectangle, types::{Matrix2d, Vec2d, Color}};

pub struct Button {
    dims: [f64; 4],
    shape: Rectangle,
    hover: bool,
}

impl Button {
    pub fn new(pos: Vec2d, width: f64, height: f64, color: Color) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let shape = Rectangle::new(color);
        Button{
            dims,
            shape,
            hover: false
        }
    }

    pub fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {

        self.shape.draw(
            self.dims,
            &piston_window::DrawState::default(),
            transform,
            graphics,
        );

    }

    pub fn check_hover (&mut self, mouse_position: Vec2d){       
        if mouse_position[0] > self.dims[0] 
        && mouse_position[0] < self.dims[0] + self.dims[2] 
        && mouse_position[1] > self.dims[1] 
        && mouse_position[1] < self.dims[1] + self.dims[3]{
            self.hover = true;
            println!("YEEEEES");
        } else{
            self.hover = false;
            println!("NOOOOOOOOOO");
        }
    }
}