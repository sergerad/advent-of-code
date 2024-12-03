use crate::{is_safe, parse};

#[tracing::instrument(skip(input))]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette::miette!(e))?;
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    Ok(format!("{}", safe_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
1 1 1 1 1";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
