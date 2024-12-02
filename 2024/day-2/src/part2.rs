use itertools::Itertools;

use crate::check_pair_safety;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let safe_count = input.lines().fold(0, |safe, line| {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        if nums
            .iter()
            .tuple_windows::<(_, _)>()
            .try_fold(None, |sign, (&a, &b)| check_pair_safety(sign, a, b))
            .is_ok()
            || nums.iter().combinations(nums.len() - 1).any(|nums| {
                nums.iter()
                    .tuple_windows::<(_, _)>()
                    .try_fold(None, |sign, (&a, &b)| check_pair_safety(sign, *a, *b))
                    .is_ok()
            })
        {
            safe + 1
        } else {
            safe
        }
    });
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
