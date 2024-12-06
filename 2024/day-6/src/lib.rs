use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
enum GameStatus {
    Running,
    Finished(usize),
    Looping,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone)]
struct Game {
    matrix: Matrix,
    guard: Position,
    direction: Direction,
    visited: HashSet<Position>,
    visited_directions: HashSet<(Position, Direction)>,
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
        let mut visited = HashSet::default();
        visited.insert(guard);
        let mut visited_directions = HashSet::default();
        visited_directions.insert((guard, Direction::default()));
        Game {
            matrix,
            guard,
            direction: Direction::default(),
            visited,
            visited_directions,
        }
    }
}

impl Game {
    #[tracing::instrument(ret)]
    fn update(&mut self) -> GameStatus {
        let move_pos = match self.direction {
            Direction::Up => {
                if self.guard.y.checked_sub(1).is_none() {
                    return GameStatus::Finished(self.visited.len());
                }
                Position {
                    x: self.guard.x,
                    y: self.guard.y - 1,
                }
            }
            Direction::Down => {
                if self.guard.y + 1 >= self.matrix.len() {
                    return GameStatus::Finished(self.visited.len());
                }
                Position {
                    x: self.guard.x,
                    y: self.guard.y + 1,
                }
            }
            Direction::Left => {
                if self.guard.x.checked_sub(1).is_none() {
                    return GameStatus::Finished(self.visited.len());
                }
                Position {
                    x: self.guard.x - 1,
                    y: self.guard.y,
                }
            }
            Direction::Right => {
                if self.guard.x + 1 >= self.matrix[0].len() {
                    return GameStatus::Finished(self.visited.len());
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
                self.visited.insert(move_pos);
                if self
                    .visited_directions
                    .contains(&(move_pos, self.direction))
                {
                    return GameStatus::Looping;
                } else {
                    self.visited_directions.insert((move_pos, self.direction));
                    self.guard = move_pos;
                }
            }
            Spot::Obstacle => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
        }
        GameStatus::Running
    }
}
