use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::opt,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse_position(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("p="),
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn parse_velocity(input: &str) -> IResult<&str, IVec2> {
    preceded(
        tag("v="),
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    terminated(
        separated_pair(parse_position, complete::space1, parse_velocity),
        opt(line_ending),
    )(input)
    .map(|(input, (position, velocity))| (input, Robot { position, velocity }))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    many1(parse_robot)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, robots) = parse(input).map_err(|e| miette::miette!("{e}"))?;
    let size = if cfg!(test) {
        IVec2::new(11, 7)
    } else {
        IVec2::new(101, 103)
    };
    let iters = 100;
    let halves = size / 2;
    let quadrants = [
        (0..halves.x, 0..halves.y),
        ((halves.x + 1)..size.x, 0..halves.y),
        (0..halves.x, (halves.y + 1)..size.y),
        ((halves.x + 1)..size.x, (halves.y + 1)..size.y),
    ];
    let sum: i32 = robots
        .iter()
        .fold([0, 0, 0, 0], |mut quads, robot| {
            let position = (robot.position + robot.velocity * iters).rem_euclid(size);
            if let Some(index) = quadrants
                .iter()
                .position(|(min, max)| min.contains(&position.x) && max.contains(&position.y))
            {
                quads[index] += 1;
            }
            quads
        })
        .iter()
        .filter(|&count| *count > 0)
        .product();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
