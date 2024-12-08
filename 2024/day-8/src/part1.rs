use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn process(input: &'static str) -> miette::Result<String> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let rows = 0i32..rows as i32;
    let cols = 0i32..cols as i32;

    let (_, mut satellites) = parse(Span::new(input)).map_err(|e| miette!(e))?;
    satellites.sort_by(|a, b| a.1.cmp(&b.1));
    let results = satellites
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk.iter().tuple_combinations().flat_map(|(a, b)| {
                let diff = a.0 - b.0;
                [a.0 + diff, b.0 - diff]
            })
        })
        .filter(|pos| cols.contains(&pos.x) && rows.contains(&pos.y))
        .unique()
        .count();
    Ok(results.to_string())
}

fn parse_alphanum_pos(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(
        take_till(|c: char| c.is_alphanum()),
        parse_alphanum_pos,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
