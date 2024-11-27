use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    // Read the input file.
    let file = File::open("./y2023/d1/input.txt")?;
    let reader = BufReader::new(file);

    // Parse the lines into a list of 2 digit, base 10 numbers.
    let sum = reader
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
        .collect::<anyhow::Result<Vec<_>>>();

    // Print the sum of the numbers.
    println!("sum: {:?}", sum?.into_iter().sum::<u32>());
    Ok(())
}
