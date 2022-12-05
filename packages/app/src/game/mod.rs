mod snake;
use crate::{
    audio,
    utils::{f64_from_usize, random_usize},
};
use snake::{Cell, Snake};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Paused,
    Won,
    Lost,
    Running,
}

#[derive(Clone)]
pub struct World {
    width: usize,
    size: usize,
    snake: snake::Snake,
    next_cell: Option<Cell>,
    reward_cell: Option<usize>,
    status: Status,
    points: usize,
    audio_system: audio::System,
    is_started: bool,
}

impl World {
    pub fn new(width: usize, height: usize, spawn_index: usize) -> World {
        let size = width * height;
        let snake = Snake::new(spawn_index, 3);
        let audio_system = audio::System::new();
        World {
            width,
            size,
            reward_cell: Some(World::gen_reward_cell(size, &snake.body)),
            snake,
            next_cell: None,
            status: Status::Paused,
            points: 0,
            audio_system,
            is_started: false,
        }
    }

    pub fn start_game(&mut self) {
        self.start_audio();
        self.audio_system.trigger("start", None);
        self.status = Status::Running;
        self.is_started = true;
    }

    pub fn pause_game(&mut self) {
        self.audio_system.trigger("pause", None);
        self.status = Status::Paused;
    }

    pub fn resume_game(&mut self) {
        if !self.is_started {
            return self.start_game();
        }
        self.audio_system.trigger("resume", None);
        self.status = Status::Running;
    }

    pub fn restart(&mut self) {
        self.audio_system.trigger("restart", None);
        self.snake = Snake::new(self.snake_head_index(), 3);
        self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));
        self.status = Status::Paused;
        self.points = 0;
    }

    pub fn start_audio(&mut self) {
        self.audio_system.start();
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn snake_body(&self) -> Vec<usize> {
        self.snake.body.iter().map(|cell| cell.0).collect()
    }

    pub fn step(&mut self) {
        if self.status != Status::Running {
            return;
        }
        let temp = self.snake.body.clone();
        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(self.snake.direction);
            }
        }
        self.audio_system
            .trigger("step", Some(f64_from_usize(self.snake.body[0].0)));

        let len = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = Cell(temp[i - 1].0);
        }

        if self.snake.body[1..len].contains(&self.snake.body[0]) {
            self.audio_system.trigger("lose", None);
            self.status = Status::Lost;
        }

        if self.reward_cell == Some(self.snake_head_index()) {
            if len < self.size {
                self.points += 1;
                self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));
                self.audio_system.trigger("eat", Some(f64_from_usize(len)));
            } else {
                self.reward_cell = None;
                self.status = Status::Won;
                self.audio_system.trigger("win", None);
            }

            self.snake.body.push(Cell(self.snake.body[1].0));
        }
    }

    pub fn game_status(&self) -> Status {
        self.status
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction == self.snake.direction {
            return;
        }
        self.audio_system
            .trigger("direction", Some(f64::from(direction as u8)));
        let next_cell = self.gen_next_snake_cell(direction);
        if self.snake.body[1] == next_cell {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    fn gen_next_snake_cell(&self, direction: Direction) -> Cell {
        let snake_idx = self.snake_head_index();
        let row = snake_idx / self.width;

        match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    Cell(treshold - self.width)
                } else {
                    Cell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    Cell(treshold + (self.width - 1))
                } else {
                    Cell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    Cell((self.size - self.width) + treshold)
                } else {
                    Cell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    Cell(treshold - ((row + 1) * self.width))
                } else {
                    Cell(snake_idx + self.width)
                }
            }
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &[Cell]) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random_usize(0, max);
            if !snake_body.contains(&Cell(reward_cell)) {
                break;
            }
        }

        reward_cell
    }
}

#[test]
fn test_world() {
    let world = World::new(8, 8, 10);
    assert_eq!(world.snake_head_index(), 10);
}

#[test]
fn test_snake() {
    let snake = Snake::new(10, 3);
    assert_eq!(snake.body[0].0, 10);
    assert_eq!(snake.body.len(), 3);
}

#[test]
fn test_gen_next_snake_cell() {
    let world = World::new(8, 8, 10);
    assert_eq!(world.gen_next_snake_cell(Direction::Right).0, 11);
    assert_eq!(world.gen_next_snake_cell(Direction::Left).0, 9);
    assert_eq!(world.gen_next_snake_cell(Direction::Up).0, 2);
    assert_eq!(world.gen_next_snake_cell(Direction::Down).0, 18);
}

#[test]
fn test_set_direction() {
    let mut world = World::new(8, 8, 10);

    world.set_direction(Direction::Up);
    assert_eq!(world.snake.direction, Direction::Up);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 2);

    world.set_direction(Direction::Right);
    assert_eq!(world.snake.direction, Direction::Right);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 11);

    world.set_direction(Direction::Down);
    assert_eq!(world.snake.direction, Direction::Down);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 18);

    world.set_direction(Direction::Left);
    assert_eq!(world.snake.direction, Direction::Left);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 9);
}
