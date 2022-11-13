use rand::Rng;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

fn random(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Copy, Clone, Debug, PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
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

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Paused,
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: GameStatus,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize, spawn_index: usize) -> World {
        println!(
            "Creating a new world with width: {}, height: {}, spawn_index: {}",
            width, height, spawn_index
        );
        let size = width * height;
        let snake = Snake::new(spawn_index, 3);
        World {
            width,
            height,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: GameStatus::Paused,
            points: 0,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn snake_body(&self) -> Vec<usize> {
        self.snake.body.iter().map(|cell| cell.0).collect()
    }

    pub fn step(&mut self) {
        match self.status {
            GameStatus::Played => {
                let temp = self.snake.body.clone();
                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }
                let len = self.snake.body.len();

                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..len].contains(&self.snake.body[0]) {
                    self.status = GameStatus::Lost
                }

                if self.reward_cell == Some(self.snake_head_index()) {
                    if len < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.status = GameStatus::Won
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            _ => {}
        }
    }

    pub fn start_game(&mut self) {
        self.status = GameStatus::Played;
    }

    pub fn pause_game(&mut self) {
        self.status = GameStatus::Paused;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(self.snake_head_index(), 3);
        self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
        self.status = GameStatus::Paused;
        self.points = 0;
    }

    pub fn game_status(&self) -> GameStatus {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            GameStatus::Won => String::from("You have won!"),
            GameStatus::Lost => String::from("You have lost!"),
            GameStatus::Played => String::from("Playing"),
            GameStatus::Paused => String::from("Paused"),
            _ => String::from("Unknown State"),
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1] == next_cell {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_index();
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
    pub fn snake_body_len(&self) -> usize {
        self.snake.body.len()
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;
        loop {
            reward_cell = random(0, max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }
}

#[test]
fn test_world() {
    let world = World::new(8, 8, 10);
    assert_eq!(world.width(), 8);
    assert_eq!(world.height(), 8);
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
    assert_eq!(world.gen_next_snake_cell(&Direction::Right).0, 11);
    assert_eq!(world.gen_next_snake_cell(&Direction::Left).0, 9);
    assert_eq!(world.gen_next_snake_cell(&Direction::Up).0, 2);
    assert_eq!(world.gen_next_snake_cell(&Direction::Down).0, 18);
}

#[test]
fn test_set_direction() {
    let mut world = World::new(8, 8, 10);

    world.set_direction(Direction::Right);
    assert_eq!(world.snake.direction, Direction::Right);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 11);

    world.set_direction(Direction::Left);
    assert_eq!(world.snake.direction, Direction::Left);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 9);

    world.set_direction(Direction::Up);
    assert_eq!(world.snake.direction, Direction::Up);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 2);

    world.set_direction(Direction::Down);
    assert_eq!(world.snake.direction, Direction::Down);
    assert_eq!(world.next_cell.as_ref().unwrap().0, 18);
}