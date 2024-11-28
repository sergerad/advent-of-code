use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn part_1() -> anyhow::Result<()> {
    // Read the input file.
    let file = File::open("./y2023/d1/input.txt")?;
    let reader = BufReader::new(file);

    // Parse the lines into a list of 2 digit, base 10 numbers.
    let nums = reader
        .lines()
        .map(|line| {
            let line = line?;

            // Filter out all non-digit characters.
            let mut digits = line.chars().filter(|c| c.is_ascii_digit());

            // Combine the first and last digits into a single number.
            let first = digits
                .next()
                .ok_or(anyhow::anyhow!("First digit not found"))?;
            let second = digits.last().unwrap_or(first);
            let cat = format!("{}{}", first, second);
            Ok(cat.parse::<u32>()?)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Print the sum of the numbers.
    println!("part 1: {:?}", nums.into_iter().sum::<u32>());
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    // Read the input file.
    let file = File::open("./y2023/d1/input.txt")?;
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
    println!("part 2: {:?}", nums.into_iter().sum::<usize>());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}
