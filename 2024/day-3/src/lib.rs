pub mod part1;
pub mod part2;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::many0,
    sequence::tuple,
    IResult,
};

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

#[tracing::instrument(skip(input), ret)]
fn parse_mul_expression(input: &str) -> IResult<&str, (i32, i32)> {
    let parser = tuple((
        tag("mul("), // Must start with "mul("
        parse_integer,
        char(','),
        parse_integer,
        char(')'), // Must end with ")"
    ));

    map(parser, |(_, a, _, b, _)| (a, b))(input)
}

//fn parse_mul_expressions(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
//    many0(
//        // Consume any characters until a "mul(" is found
//        nom::sequence::preceded(
//            nom::bytes::complete::take_until("mul("),
//            parse_mul_expression,
//        ),
//    )(input)
//}

fn parse_mul_expressions(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(
        // Continue parsing after finding mul(x,y), even after other characters
        alt((
            map(parse_mul_expression, Some),
            map(nom::character::complete::anychar, |_| None),
        )),
    )(input)
    .map(|(rest, results)| (rest, results.into_iter().flatten().collect()))
}
