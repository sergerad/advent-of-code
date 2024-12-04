use std::vec;

use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

type Row = Vec<char>;
type Matrix = Vec<Row>;
struct IndexedMatrix {
    matrix: Matrix,
    index: Vec<(i32, i32)>,
}

fn parse(input: &str) -> IResult<&str, Matrix> {
    separated_list1(line_ending, complete::alpha1)(input).map(|(input, rows)| {
        (
            input,
            rows.into_iter().map(|row| row.chars().collect()).collect(),
        )
    })
}

impl IndexedMatrix {
    fn new(matrix: Matrix) -> IndexedMatrix {
        let mut index = vec![];
        let cols = matrix[0].len();
        let rows = matrix.len();
        for (y, row) in matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'A' && x != 0 && y != 0 && x != cols - 1 && y != rows - 1 {
                    index.push((x as i32, y as i32));
                }
            }
        }
        IndexedMatrix { matrix, index }
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, matrix) = parse(input).map_err(|e| miette::miette!(e))?;
    let matrix = IndexedMatrix::new(matrix);
    let mas_count: usize = matrix
        .index
        .iter()
        .map(|(x, y)| {
            DIRECTIONS
                .iter()
                .filter(|(dx, dy)| {
                    let m_x = x + dx;
                    let m_y = y + dy;
                    if matrix.matrix[m_y as usize][m_x as usize] == 'M' {
                        let s_x = x - dx;
                        let s_y = y - dy;
                        matrix.matrix[s_y as usize][s_x as usize] == 'S'
                    } else {
                        false
                    }
                })
                .count()
        })
        .filter(|&count| count == 2)
        .count();
    Ok(format!("{}", mas_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
