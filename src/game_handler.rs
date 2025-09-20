use macroquad::time::get_time;

use crate::block::Block;
use crate::snake::Snake;

pub struct Game {
    pub snake: Snake,
    pub apple: Block,
    pub timer: f64,
    pub lost: bool,
}

impl Game {
    pub fn new() -> Self {

        Self {
            snake: Snake::new(),
            apple: Block::random(),
            timer: get_time(),
            lost: false,
        }
    }

    pub fn move_snake(mut self, extend: bool) -> Self {
        self.snake = self.snake.r#move(extend);
        self
    }

    pub fn check_lose(&mut self) {
        if (self.snake.head.location.0 >= 20) ||
        (self.snake.head.location.0 <= -1) ||
        (self.snake.head.location.1 >= 20) ||
        (self.snake.head.location.1 <= -1) {
            self.lost = true;
            return;
        }
                

        for block in self.snake.blocks.iter() {
            if block.check_overlap(&self.snake.head) {
                self.lost = true;
                return;
            }
        }
    }

}
