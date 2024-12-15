use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of, satisfy},
    multi::{count, many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};

use nom_locate::{position, LocatedSpan};
pub type Span<'a> = LocatedSpan<&'a str>;

fn parse_char_pos(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c != '\n')(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse_map(input: Span) -> IResult<Span, HashMap<IVec2, char>> {
    terminated(
        separated_list1(line_ending, many1(parse_char_pos)).map(|v| {
            let mut map = HashMap::new();
            v.iter().flatten().for_each(|(pos, c)| {
                map.insert(*pos, *c);
            });
            map
        }),
        count(line_ending, 2),
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of("<^>v")))(input)
}

fn parse(input: &str) -> miette::Result<(HashMap<IVec2, char>, Vec<char>)> {
    let span = Span::new(input);
    let (span, map) = parse_map(span).map_err(|e| miette::miette!("{e}"))?;
    let (_, moves) = parse_moves(span.to_string().as_str()).map_err(|e| miette::miette!("{e}"))?;
    Ok((map, moves.iter().flatten().cloned().collect()))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut map, moves) = parse(input)?;
    let mut robot_pos = *map.iter().find(|(_, c)| **c == '@').unwrap().0;
    for mv in moves {
        let next_robot_pos = match mv {
            '^' => IVec2::new(robot_pos.x, robot_pos.y - 1),
            'v' => IVec2::new(robot_pos.x, robot_pos.y + 1),
            '<' => IVec2::new(robot_pos.x - 1, robot_pos.y),
            '>' => IVec2::new(robot_pos.x + 1, robot_pos.y),
            _ => panic!("Invalid move"),
        };
        let next_robot_pos_char = map.get(&next_robot_pos).unwrap();
        match next_robot_pos_char {
            '.' => {
                map.insert(robot_pos, '.');
                map.insert(next_robot_pos, '@');
                robot_pos = next_robot_pos;
            }
            'O' => {
                // Find next char that is not 'O'
                let mut ending_pos = next_robot_pos;
                let mut ending_pos_char = 'O';
                while ending_pos_char == 'O' {
                    ending_pos = match mv {
                        '^' => IVec2::new(ending_pos.x, ending_pos.y - 1),
                        'v' => IVec2::new(ending_pos.x, ending_pos.y + 1),
                        '<' => IVec2::new(ending_pos.x - 1, ending_pos.y),
                        '>' => IVec2::new(ending_pos.x + 1, ending_pos.y),
                        _ => panic!("Invalid move"),
                    };
                    ending_pos_char = *map.get(&ending_pos).unwrap();
                }
                match ending_pos_char {
                    '.' => {
                        map.insert(robot_pos, '.');
                        map.insert(next_robot_pos, '@');
                        map.insert(ending_pos, 'O');
                        robot_pos = next_robot_pos;
                    }
                    '#' => {}
                    c => panic!("Invalid map val {c}"),
                }
            }
            '#' => {}
            _ => panic!("Invalid map val"),
        }
    }
    let sum = map
        .iter()
        .filter(|(_, c)| **c == 'O')
        .map(|(pos, _)| pos.x + pos.y * 100)
        .sum::<i32>();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
