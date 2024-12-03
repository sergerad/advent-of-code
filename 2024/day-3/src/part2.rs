use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction).map(|(_, instruction)| instruction))(input)
}

enum ProcessState {
    Enabled,
    Disabled,
}

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette::miette!(e))?;
    let sum: u32 = instructions
        .iter()
        .fold(
            (ProcessState::Enabled, 0u32),
            |(state, mul_sum), instruction| match instruction {
                Instruction::Mul(a, b) => {
                    if let ProcessState::Enabled = state {
                        (state, mul_sum + a * b)
                    } else {
                        (state, mul_sum)
                    }
                }
                Instruction::Do => (ProcessState::Enabled, mul_sum),
                Instruction::Dont => (ProcessState::Disabled, mul_sum),
            },
        )
        .1;
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
