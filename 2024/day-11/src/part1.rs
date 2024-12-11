#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut nums: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (0..25).for_each(|_| {
        let mut i = 0;
        while i < nums.len() {
            let num = nums[i];
            if num == 0 {
                nums[i] = 1;
            } else {
                let num = num.to_string();
                if num.len() % 2 == 0 {
                    let num_split = num.split_at(num.len() / 2);
                    nums[i] = num_split.0.parse::<usize>().unwrap();
                    nums.insert(i + 1, num_split.1.parse::<usize>().unwrap());
                    i += 1;
                } else {
                    nums[i] *= 2024;
                }
            }
            i += 1;
        }
    });
    Ok(nums.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
