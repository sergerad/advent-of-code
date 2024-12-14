use glam::UVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Machine {
    button_a: UVec2,
    button_b: UVec2,
    prize: UVec2,
}

fn parse_button_a(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Button A: X+"),
        separated_pair(complete::u32, tag(", Y+"), complete::u32).map(|(x, y)| UVec2::new(x, y)),
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Button B: X+"),
        separated_pair(complete::u32, tag(", Y+"), complete::u32).map(|(x, y)| UVec2::new(x, y)),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, UVec2> {
    preceded(
        tag("Prize: X="),
        separated_pair(complete::u32, tag(", Y="), complete::u32).map(|(x, y)| UVec2::new(x, y)),
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (button_a, button_b, prize)) = tuple((
        terminated(parse_button_a, line_ending),
        terminated(parse_button_b, line_ending),
        parse_prize,
    ))(input)?;

    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}
fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(count(line_ending, 2), parse_machine)(input)
}

#[tracing::instrument]
pub fn process(_input: &'static str) -> miette::Result<String> {
    let (_, machines) = parse(_input).map_err(|e| miette::miette!(e))?;
    let tokens_spent: u32 = machines
        .iter()
        .map(|machine| {
            let mut tokens_spent = 0;
            for combo_size in 2..200 {
                let mut smallest_cost: Option<u32> = None;
                for combo in [(machine.button_a, 3), (machine.button_b, 1)]
                    .iter()
                    .combinations_with_replacement(combo_size)
                {
                    let (sum, cost) = combo.iter().fold((UVec2::ZERO, 0), |(sum, cost), combo| {
                        (sum + combo.0, cost + combo.1)
                    });
                    if sum == machine.prize {
                        tokens_spent += cost;
                        match smallest_cost {
                            Some(latest_smallest_cost) => {
                                if cost < latest_smallest_cost {
                                    smallest_cost = Some(cost);
                                }
                            }
                            None => {
                                smallest_cost = Some(cost);
                            }
                        }
                        break;
                    }
                }
                if let Some(cost) = smallest_cost {
                    tokens_spent += cost;
                }
            }
            tokens_spent / 2
        })
        .sum();
    Ok(tokens_spent.to_string())
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
