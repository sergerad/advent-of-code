#[derive(Debug, Clone, Copy)]
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
    let free_count = input.iter().filter(|&e| matches!(e, Elem::Dot)).count();
    let back = input
        .iter()
        .rev()
        .filter(|&e| matches!(e, Elem::Id(_)))
        .take(free_count);
    let compacted = back
        .fold((input.clone(), 0), |(mut elems, mut i), &e| {
            while let Some(Elem::Id(_)) = elems.get(i) {
                i += 1;
            }
            elems[i] = e;
            (elems, i)
        })
        .0;
    let compacted = &compacted[..compacted.len() - free_count];
    let end_space = vec![Elem::Dot; free_count];
    let compacted = [compacted, &end_space[..]].concat();
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
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
