use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    // Read the input file.
    let file = File::open("./day-1/input.txt")?;
    let reader = BufReader::new(file);

    const NUMBERS: &[&str] = &[
        "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six",
        "7", "seven", "8", "eight", "9", "nine",
    ];

    let nums = reader
        .lines()
        .map(|line| {
            let line = line?;
            // Find the first and last instances of a number in the line.
            let (first, last) = NUMBERS
                .iter()
                .enumerate()
                .flat_map(|(num_index, &num)| {
                    // Match all instances of the number in the line.
                    line.match_indices(num)
                        .map(move |(line_index, _)| (line_index, num_index / 2))
                })
                .minmax()
                .into_option()
                .unwrap();
            // Shift first by 1 decimal place and add last.
            Ok(first.1 * 10 + last.1)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Print the sum of the numbers.
    println!("{:?}", nums.into_iter().sum::<usize>());
    Ok(())
}
