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
    cols: usize,
    rows: usize,
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
        for (y, row) in matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'X' {
                    index.push((x as i32, y as i32));
                }
            }
        }
        let cols = matrix[0].len();
        let rows = matrix.len();
        IndexedMatrix {
            matrix,
            index,
            cols,
            rows,
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

#[tracing::instrument]
pub fn process(input: &'static str) -> miette::Result<String> {
    let (_, matrix) = parse(input).map_err(|e| miette::miette!(e))?;
    let matrix = IndexedMatrix::new(matrix);
    let xmas_count = matrix.index.iter().fold(0, |mut xmas_count, (x, y)| {
        for (dx, dy) in DIRECTIONS.iter() {
            let mut next = 'M';
            let mut x = x + dx;
            let mut y = y + dy;
            while x >= 0 && x < matrix.cols as i32 && y >= 0 && y < matrix.rows as i32 {
                if matrix.matrix[y as usize][x as usize] == next {
                    match next {
                        'M' => {
                            next = 'A';
                        }
                        'A' => {
                            next = 'S';
                        }
                        'S' => {
                            xmas_count += 1;
                            break;
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
        xmas_count
    });
    Ok(format!("{}", xmas_count))
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
