use crate::parse_mul_expressions;

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, tuples) = parse_mul_expressions(input).map_err(|e| miette::miette!(e))?;
    let sum: i32 = tuples.iter().map(|(a, b)| a * b).sum();
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
