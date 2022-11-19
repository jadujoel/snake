
use crate::game::Direction;

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(pub usize);

#[derive(PartialEq, Clone)]
pub struct Snake {
    pub body: Vec<SnakeCell>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}
