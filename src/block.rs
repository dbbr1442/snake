use macroquad::color::Color;
use macroquad::rand::gen_range;
use macroquad::shapes::draw_rectangle;

pub struct Block {
    pub location: (i32, i32),
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Self { location: (x, y) }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle(self.location.0 as f32*30.0, self.location.1 as f32*30.0, 30.0, 30.0, color);
    }

    pub fn random() -> Self {
        let x = gen_range(0, 20);
        let y = gen_range(0, 20);
        Self { location: (x, y) }
    }

    pub fn check_overlap(&self, other: &Self) -> bool {
        if self.location == other.location {
            return true;
        } else {
            return false;
        }
    }
}
