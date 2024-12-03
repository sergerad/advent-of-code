use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
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

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_mul).map(|(_, instruction)| instruction))(input)
}

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette::miette!(e))?;
    let sum: u32 = instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
