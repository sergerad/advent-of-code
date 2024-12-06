use crate::Game;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut game = Game::from(input);

    loop {
        if let Some(count) = game.update() {
            return Ok(count.to_string());
        }
    }
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
