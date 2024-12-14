use glam::{DMat2, U64Vec2};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Machine {
    a: U64Vec2,
    b: U64Vec2,
    prize: U64Vec2,
}

impl Machine {
    fn solve(&self) -> Result<U64Vec2, ()> {
        let ab = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let d_ab = ab.determinant();
        let ac = DMat2::from_cols_array(&[
            self.prize.x as f64,
            self.prize.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let d_ac = ac.determinant();

        let bc = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.prize.x as f64,
            self.prize.y as f64,
        ]);
        let d_bc = bc.determinant();
        let x = d_ac / d_ab;
        let y = d_bc / d_ab;
        if x.trunc() != x || y.trunc() != y {
            return Err(());
        }
        let max = if cfg!(test) { 100f64 } else { f64::INFINITY };

        if x > max || y > max {
            Err(())
        } else {
            Ok(U64Vec2::new(x as u64, y as u64))
        }
    }
}

fn parse_button_a(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Button A: X+"),
        separated_pair(complete::u64, tag(", Y+"), complete::u64).map(|(x, y)| U64Vec2::new(x, y)),
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::u64, tag(", Y+"), complete::u64).map(|(x, y)| U64Vec2::new(x, y)),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, U64Vec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::u64, tag(", Y="), complete::u64).map(|(x, y)| {
            U64Vec2::new(
                x + if cfg!(test) { 0 } else { 10000000000000 },
                y + if cfg!(test) { 0 } else { 10000000000000 },
            )
        }),
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, prize)) = tuple((
        terminated(parse_button_a, line_ending),
        terminated(parse_button_b, line_ending),
        parse_prize,
    ))(input)?;

    Ok((input, Machine { a, b, prize }))
}
fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(count(line_ending, 2), parse_machine)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse(input).map_err(|e| miette::miette!("{e}"))?;

    let sum: u64 = machines
        .iter()
        .filter_map(|machine| {
            machine
                .solve()
                .map(|solved| (solved * U64Vec2::new(3, 1)).element_sum())
                .ok()
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
