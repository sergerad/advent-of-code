use std::collections::HashSet;

use itertools::Itertools;

use nom::{
    bytes::complete::take_while,
    character::complete::{self, char, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Ordering = (u32, u32);
type Updates = Vec<u32>;

fn parse_ordering(input: &str) -> IResult<&str, Ordering> {
    separated_pair(complete::u32, char('|'), complete::u32)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Updates> {
    separated_list1(complete::char(','), complete::u32)(input)
}

fn parse(input: &str) -> IResult<&str, (HashSet<Ordering>, Vec<Updates>)> {
    let (input, orderings) = fold_many1(
        terminated(parse_ordering, line_ending),
        HashSet::default,
        |mut set: HashSet<(u32, u32)>, ordering| {
            set.insert(ordering);
            set
        },
    )(input)?;
    let (input, _) = take_while(|c| c == '\n')(input)?;
    let (input, updates) = separated_list1(line_ending, parse_updates)(input)?;
    Ok((input, (orderings, updates)))
}

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, (orderings, updates)) = parse(input).map_err(|e| miette::miette!(e))?;
    let sum: u32 = updates
        .into_iter()
        .filter(|update| {
            update
                .iter()
                .tuple_combinations::<(_, _)>()
                .all(|update_pair| !orderings.contains(&(*update_pair.1, *update_pair.0)))
        })
        .fold(0, |middle_sum, updates| {
            updates
                .iter()
                .enumerate()
                .fold(middle_sum, |middle_sum, (index, update)| {
                    if index == updates.len() / 2 {
                        middle_sum + update
                    } else {
                        middle_sum
                    }
                })
        });

    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
