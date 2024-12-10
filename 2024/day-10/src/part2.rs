use glam::IVec2;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use rayon::prelude::*;

pub type Span<'a> = LocatedSpan<&'a str>;

fn parse_num_pos(input: Span) -> IResult<Span, (IVec2, i32)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_numeric())(input)?;
    Ok((
        input,
        (
            IVec2::new(x, y),
            c.to_digit(10).unwrap().try_into().unwrap(),
        ),
    ))
}

fn parse(input: Span) -> IResult<Span, Vec<Vec<(IVec2, i32)>>> {
    separated_list1(line_ending, many1(parse_num_pos))(input)
}

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, trail_map) = parse(Span::new(input)).map_err(|e| miette::miette!(e))?;

    let trail_peaks = trail_map.par_iter().flatten().filter(|(_, num)| *num == 9);
    let sum: usize = trail_peaks
        .map(|(head_pos, _num)| {
            let mut trails_found = 0;
            walk(
                &trail_map,
                *head_pos,
                IVec2::new(0, 0),
                0,
                &mut trails_found,
            );
            trails_found
        })
        .sum();
    Ok(sum.to_string())
}

fn walk(
    trail_map: &Vec<Vec<(IVec2, i32)>>,
    position: IVec2,
    direction_from: IVec2,
    steps: usize,
    trails_found: &mut usize,
) {
    [
        IVec2::new(0, -1),
        IVec2::new(0, 1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ]
    .iter()
    .filter(|&direction| *direction != -direction_from)
    .for_each(|direction| {
        let next_pos = position + direction;
        if next_pos.x < 0 || next_pos.y < 0 {
            return;
        }
        if next_pos.x >= trail_map[0].len() as i32 || next_pos.y >= trail_map.len() as i32 {
            return;
        }
        let num = trail_map[position.y as usize][position.x as usize].1;
        let next_num = trail_map[next_pos.y as usize][next_pos.x as usize].1;
        if steps == 8 && next_num == 0 {
            *trails_found += 1;
            return;
        }
        if next_num != num - 1 {
            return;
        }
        walk(trail_map, next_pos, *direction, steps + 1, trails_found);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
