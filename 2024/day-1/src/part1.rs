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
    let zip = heap_a
        .into_sorted_vec()
        .into_iter()
        .zip(heap_b.into_sorted_vec());

    // Calculate the sum of the absolute differences.
    let sum = zip.map(|(a, b)| (a - b).abs()).sum::<i32>();
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
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
