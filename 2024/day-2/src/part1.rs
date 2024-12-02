use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let safe_count = input.lines().fold(0, |safe, line| {
        if line
            .split_whitespace()
            .tuple_windows::<(_, _)>()
            .try_fold(i32::MAX, |sign, (a, b)| {
                let a = a.parse::<i32>().unwrap();
                let b = b.parse::<i32>().unwrap();
                let diff = b - a;
                let diff_sign = diff.signum();
                match diff.abs() {
                    1..=3 => {
                        if sign == i32::MAX || sign == diff_sign {
                            Ok(diff_sign)
                        } else {
                            Err(miette::miette!("{line} wrong sign {diff} ({a}, {b})"))
                        }
                    }
                    _ => Err(miette::miette!("{line} wrong diff {diff} ({a}, {b})")),
                }
            })
            .is_ok()
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
