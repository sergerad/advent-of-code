use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

pub mod part1;
pub mod part2;

pub type Report = Vec<i32>;

pub fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

pub fn is_safe(report: &Report) -> bool {
    report
        .iter()
        .tuple_windows::<(_, _)>()
        .try_fold(None, |sign, (a, b)| check_pair_safety(sign, *a, *b))
        .is_ok()
}

fn check_pair_safety(sign: Option<i32>, a: i32, b: i32) -> miette::Result<Option<i32>> {
    let diff = b - a;
    let diff_sign = diff.signum();
    match diff.abs() {
        1..=3 => {
            if sign.is_none() || sign.unwrap() == diff_sign {
                Ok(Some(diff_sign))
            } else {
                Err(miette::miette!("wrong sign {diff} ({a}, {b})"))
            }
        }
        _ => Err(miette::miette!("wrong diff {diff} ({a}, {b})")),
    }
}
