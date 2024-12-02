pub mod part1;
pub mod part2;

pub fn check_pair_safety(sign: Option<i32>, a: i32, b: i32) -> miette::Result<Option<i32>> {
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
