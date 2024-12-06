use crate::{Game, GameStatus, Spot};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let game = Game::from(input);

    let mut count = 0;
    for y in 0..game.matrix.len() {
        for x in 0..game.matrix[y].len() {
            match game.matrix[y][x] {
                Spot::Nothing => {
                    let mut game = game.clone();
                    game.matrix[y][x] = Spot::Obstacle;
                    loop {
                        match game.update() {
                            GameStatus::Finished(_) => {
                                break;
                            }
                            GameStatus::Looping => {
                                count += 1;
                                break;
                            }
                            GameStatus::Running => {}
                        }
                    }
                }
                Spot::Obstacle => {}
            }
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
