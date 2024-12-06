use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Debug)]
enum Spot {
    Nothing,
    Obstacle,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '^' | '.' => Self::Nothing,
            '#' => Self::Obstacle,
            _ => panic!("Invalid spot"),
        }
    }
}

type Row = Vec<Spot>;

type Matrix = Vec<Row>;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

#[derive(Debug, Default)]
struct Game {
    matrix: Matrix,
    guard: Position,
    direction: Direction,
    visited: HashSet<Position>,
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let mut guard = Position::default();
        let matrix = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            guard.x = x;
                            guard.y = y;
                        }
                        Spot::from(c)
                    })
                    .collect()
            })
            .collect();
        Game {
            matrix,
            guard,
            direction: Direction::default(),
            visited: HashSet::default(),
        }
    }
}

impl Game {
    #[tracing::instrument(ret)]
    fn update(&mut self) -> Option<usize> {
        let move_pos = match self.direction {
            Direction::Up => {
                if self.guard.y.checked_sub(1).is_none() {
                    return Some(self.visited.len());
                }
                Position {
                    x: self.guard.x,
                    y: self.guard.y - 1,
                }
            }
            Direction::Down => {
                if self.guard.y + 1 >= self.matrix.len() {
                    return Some(self.visited.len());
                }
                Position {
                    x: self.guard.x,
                    y: self.guard.y + 1,
                }
            }
            Direction::Left => {
                if self.guard.x.checked_sub(1).is_none() {
                    return Some(self.visited.len());
                }
                Position {
                    x: self.guard.x - 1,
                    y: self.guard.y,
                }
            }
            Direction::Right => {
                if self.guard.x + 1 >= self.matrix[0].len() {
                    return Some(self.visited.len());
                }
                Position {
                    x: self.guard.x + 1,
                    y: self.guard.y,
                }
            }
        };
        let next_spot = &self.matrix[move_pos.y][move_pos.x];
        match next_spot {
            Spot::Nothing => {
                println!("Moving to {:?}", move_pos);
                self.visited.insert(move_pos);
                self.guard = move_pos;
            }
            Spot::Obstacle => {
                println!("Obstacle at {:?}", move_pos);
                self.direction = match self.direction {
                    Direction::Up => {
                        println!("moving right");
                        Direction::Right
                    }
                    Direction::Down => {
                        println!("moving left");
                        Direction::Left
                    }
                    Direction::Left => {
                        println!("moving up");
                        Direction::Up
                    }
                    Direction::Right => {
                        println!("moving down");
                        Direction::Down
                    }
                };
            }
        }
        None
    }
}
