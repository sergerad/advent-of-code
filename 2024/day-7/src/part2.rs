use itertools::Itertools;

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let sum = parts.next().unwrap().parse().unwrap();
            let factors = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect();
            (sum, factors)
        })
        .collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let input = parse(input);
    let count: u64 = input
        .par_iter()
        .filter_map(|(sum, factors)| {
            let count = factors.len() - 1;
            (0..count)
                .map(|_| vec![Operation::Add, Operation::Multiply, Operation::Concat])
                .multi_cartesian_product()
                .any(|operations| {
                    let mut operations = operations.into_iter();
                    factors
                        .iter()
                        .copied()
                        .reduce(|a, b| match operations.next().unwrap() {
                            Operation::Add => a + b,
                            Operation::Multiply => a * b,
                            Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
                        })
                        .unwrap()
                        == *sum
                })
                .then_some(sum)
        })
        .sum();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
