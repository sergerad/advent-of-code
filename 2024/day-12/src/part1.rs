use glam::IVec2;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

fn parse_char_pos(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphabetic())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<Vec<(IVec2, char)>>> {
    separated_list1(line_ending, many1(parse_char_pos))(input)
}
#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, matrix) = parse(Span::new(input)).map_err(|e| miette::miette!(e))?;
    let rows = matrix.len();
    let cols = matrix[0].len();
    //for row in matrix {
    //    for (pos, c) in row {
    //        print!("{c}");
    //    }
    //    println!("")
    //}
    let mut chunks = matrix.iter().flatten().collect::<Vec<_>>();
    chunks.sort_by(|&a, &b| a.1.cmp(&b.1));
    let sum = chunks
        .chunk_by(|&a, &b| a.1 == b.1)
        .map(|blocks| {
            let block_sum = blocks
                .iter()
                .map(|(pos, c)| {
                    [
                        IVec2::new(-1, 0),
                        IVec2::new(1, 0),
                        IVec2::new(0, -1),
                        IVec2::new(0, 1),
                    ]
                    .into_iter()
                    .map(|direction| {
                        let new_pos = *pos + direction;
                        if new_pos.x < 0
                            || new_pos.y < 0
                            || new_pos.x >= cols as i32
                            || new_pos.y >= rows as i32
                        {
                            1
                        } else {
                            if matrix[new_pos.y as usize][new_pos.x as usize].1 == *c {
                                0
                            } else {
                                1
                            }
                        }
                    })
                    .sum::<i32>()
                })
                .sum::<i32>();
            let block_len = blocks.len() as i32;
            println!("block {}: {block_sum}, block_len: {block_len}", blocks[0].1);
            block_sum * block_len
        })
        .sum::<i32>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        //        let input = "AAAA
        //BBCD
        //BBCC
        //EEEC";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
