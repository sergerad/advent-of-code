use itertools::Itertools;

use crate::{check_pair_safety, is_safe, parse};

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette::miette!(e))?;
    let safe_count = reports
        .iter()
        .filter(|&report| {
            if is_safe(report) {
                true
            } else {
                report.iter().combinations(report.len() - 1).any(|nums| {
                    nums.iter()
                        .tuple_windows::<(_, _)>()
                        .try_fold(None, |sign, (&a, &b)| check_pair_safety(sign, *a, *b))
                        .is_ok()
                })
            }
        })
        .count();
    Ok(format!("{}", safe_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative() -> miette::Result<()> {
        let input = "1 2 7 8 9
9 7 6 2 1
1 1 1 1 8";
        assert_eq!("0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_positive() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(input.lines().count(), process(input)?.parse().unwrap());
        Ok(())
    }
}
