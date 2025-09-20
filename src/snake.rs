use std::collections::LinkedList;

use macroquad::color::{DARKGREEN, GREEN};

use crate::block::Block;

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        let result = match self {
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        };

        return result;
    }

    fn as_transfrom(&self) -> (i32, i32) {
        let transform: (i32, i32) = match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        return transform;
    }
}

pub struct Snake {
    pub direction: Direction,
    pub blocks: LinkedList<Block>,
    pub head: Block,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            direction: Direction::East,
            blocks: LinkedList::new(),
            head: Block::new(1, 10), 
        }
    }

    pub fn r#move(mut self, extend: bool) -> Self {
        let transform = self.direction.as_transfrom();

        let new_x = self.head.location.0 + transform.0;
        let new_y = self.head.location.1 + transform.1;
        let new_block = Block::new(new_x, new_y);

        self.blocks.push_back(self.head);
        if !extend { self.blocks.pop_front(); };
        self.head = new_block;

        return self;
    }

    pub fn rotate(&mut self, new_dir: Direction) {
        if new_dir != self.direction.opposite() {
            self.direction = new_dir;
        }    
    }

    pub fn draw(&self) {
        for block in self.blocks.iter() {
            block.draw(GREEN);
        }

        self.head.draw(DARKGREEN);
    }

}