use std::collections::BinaryHeap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse the input file into two heaps.
    let (heap_a, heap_b) = input.lines().fold(
        (BinaryHeap::new(), BinaryHeap::new()),
        |(mut heap_a, mut heap_b), line| {
            // Parse the line into two numbers.
            if let Some((a, b)) = line.split_once("   ") {
                // Parse the numbers.
                let a = a.parse::<i32>().unwrap();
                let b = b.parse::<i32>().unwrap();
                // Insert the numbers into the lists.
                heap_a.push(a);
                heap_b.push(b);
            }
            (heap_a, heap_b)
        },
    );

    // Convert the heaps into sorted vecs and zip them together.
    let sorted_a = heap_a.into_sorted_vec();
    let sorted_b = heap_b.into_sorted_vec();

    // Count the number of times each element in first list appears in second list.
    let counts: Vec<_> = sorted_a
        .iter()
        .map(|a| {
            sorted_b.iter().take_while(|&b| a >= b).fold(
                0,
                |acc, b| {
                    if a == b {
                        acc + 1
                    } else {
                        acc
                    }
                },
            )
        })
        .collect();

    // Multiply each element in the first list by the number of times it appears in the second list.
    let zip = sorted_a.iter().zip(counts);
    let sum = zip.map(|(a, count)| a * count).sum::<i32>();
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
