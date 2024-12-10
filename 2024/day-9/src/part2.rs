#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elem {
    Id(usize),
    Dot,
}

impl std::fmt::Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Id(id) => write!(f, "{}", id),
            Elem::Dot => write!(f, "."),
        }
    }
}

fn expand(input: &str) -> miette::Result<Vec<Elem>> {
    let expanded = input
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (index, c)| {
            let num = c.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                let id = index / 2;
                acc.extend(vec![Elem::Id(id); num]);
            } else {
                acc.extend(vec![Elem::Dot; num]);
            }
            acc
        });

    Ok(expanded)
}

fn compact(input: Vec<Elem>) -> miette::Result<Vec<Elem>> {
    let mut compacted = input
        .iter()
        .enumerate()
        .rev()
        .filter(|&e| matches!(e.1, Elem::Id(_)))
        .collect::<Vec<_>>()
        .chunk_by(|&a, &b| a.1 == b.1)
        .fold(input.clone(), |mut elems, chunk| {
            let mut i = 0usize;
            while i < elems.len() && i < chunk[0].0 {
                let end = i + chunk.len();
                if elems[i..end].iter().all(|&e| matches!(e, Elem::Dot)) {
                    (i..end).for_each(|i| elems[i] = *chunk[0].1);
                    chunk.iter().for_each(|(i, _)| elems[*i] = Elem::Dot);
                    return elems;
                }
                i += 1;
            }
            elems
        });
    compacted.truncate(input.len());
    Ok(compacted)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let expanded = expand(input)?;
    let compacted = compact(expanded)?;
    let checksum = compacted
        .iter()
        .enumerate()
        .fold(0, |sum, (index, e)| match e {
            Elem::Dot => sum,
            Elem::Id(id) => sum + id * index,
        });
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
