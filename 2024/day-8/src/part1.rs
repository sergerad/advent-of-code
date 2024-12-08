use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct World {
    matrix: Vec<Vec<char>>,
    frequencies: HashMap<char, Vec<Position>>,
}

fn parse(input: &str) -> World {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let frequencies: HashMap<char, Vec<Position>> =
        matrix
            .iter()
            .enumerate()
            .fold(HashMap::default(), |frequencies, (y, row)| {
                row.iter()
                    .enumerate()
                    .fold(frequencies, |mut frequencies, (x, c)| {
                        if c != &'.' {
                            frequencies.entry(*c).or_default().push(Position {
                                x: x as isize,
                                y: y as isize,
                            });
                        }
                        frequencies
                    })
            });
    World {
        matrix,
        frequencies,
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let world = parse(input);
    let cols = world.matrix[0].len() as isize;
    let rows = world.matrix.len() as isize;
    let anti_nodes: HashSet<Position> =
        world
            .frequencies
            .into_iter()
            .fold(HashSet::default(), |mut anti, (_c, positions)| {
                positions.iter().tuple_combinations().for_each(|(a, b)| {
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;

                    if b.x + dx < cols && b.y + dy < rows {
                        anti.insert(Position {
                            x: b.x + dx,
                            y: b.y + dy,
                        });
                    }
                    if a.x - dx >= 0 && a.y - dy >= 0 {
                        anti.insert(Position {
                            x: a.x - dx,
                            y: a.y - dy,
                        });
                    }
                });
                anti
            });
    Ok(anti_nodes.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
