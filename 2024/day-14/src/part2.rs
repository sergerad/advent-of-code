use glam::IVec2;
use itertools::Itertools;
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
    let (_, mut robots) = parse(input).map_err(|e| miette::miette!("{e}"))?;

    let size = IVec2::new(101, 103);
    let mut i = 0;
    let last_step = loop {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(size);
        }
        i += 1;
        if robots
            .iter()
            .map(|Robot { position, .. }| position)
            .all_unique()
        {
            break i;
        }
    };

    Ok(last_step.to_string())
}
