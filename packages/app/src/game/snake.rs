use crate::game::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Cell(pub usize);

#[derive(PartialEq, Eq, Clone)]
pub struct Snake {
    pub body: Vec<Cell>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(Cell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}
