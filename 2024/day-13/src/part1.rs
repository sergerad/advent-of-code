use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{self, newline},
    multi::{count, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Machine {
    button_a: IVec2,
    button_b: IVec2,
    prize: IVec2,
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, a_x) = preceded(tag("Button A: X+"), complete::i32)(input)?;
    let (input, a_y) = preceded(tag(", Y+"), complete::i32)(input)?;
    let (input, _) = take(1u32)(input)?;
    let (input, b_x) = preceded(tag("Button B: X+"), complete::i32)(input)?;
    let (input, b_y) = preceded(tag(", Y+"), complete::i32)(input)?;

    let (input, _) = take(1u32)(input)?;
    let (input, prize_x) = preceded(tag("Prize: X="), complete::i32)(input)?;
    let (input, prize_y) = preceded(tag(", Y="), complete::i32)(input)?;
    Ok((
        input,
        Machine {
            button_a: IVec2::new(a_x, a_y),
            button_b: IVec2::new(b_x, b_y),
            prize: IVec2::new(prize_x, prize_y),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    many1(terminated(parse_machine, count(newline, 2)))(input)
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
                for combo in [(machine.button_a, true), (machine.button_b, false)]
                    .iter()
                    .combinations_with_replacement(combo_size)
                {
                    let (sum, cost) = combo.iter().fold((IVec2::ZERO, 0), |(sum, cost), combo| {
                        let sum = sum + combo.0;
                        let cost = match combo.1 {
                            true => cost + 3,
                            false => cost + 1,
                        };
                        (sum, cost)
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
Prize: X=18641, Y=10279

";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
