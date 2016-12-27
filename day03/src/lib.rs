extern crate base;
use base::{Part, ProblemSolver};

use std::error::Error;
use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day03)
}

struct Day03;

impl ProblemSolver for Day03 {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        let triangles = match part {
            Part::One => parse_input_part1(input)?,
            Part::Two => parse_input_part2(input)?,
        };
        Ok(count_triangles(&triangles).to_string())
    }
}

fn parse_input_part1(input: &str) -> Result<Vec<(u32, u32, u32)>, String> {
    let lines = input.lines();
    base::utils::any_err(lines.map(parse_line))
}

fn parse_line(line: &str) -> Result<(u32, u32, u32), String> {
    let line = line.trim();
    let strs = line.split_whitespace().collect::<Vec<&str>>();
    let a_str = strs[0];
    let b_str = strs[1];
    let c_str = strs[2];

    let a = u32::from_str(a_str).unwrap();
    let b = u32::from_str(b_str).unwrap();
    let c = u32::from_str(c_str).unwrap();

    Ok((a, b, c))
}

fn parse_input_part2(input: &str) -> Result<Vec<(u32, u32, u32)>, String> {
    unimplemented!()
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Copy
{
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed: Vec<Vec<T>> = Vec::with_capacity(cols);
    for col in 0..cols {
        transposed.push(Vec::with_capacity(rows));
        for row in 0..rows {
            transposed[col].push(matrix[row][col]);
        }
    }
    transposed
}

fn count_triangles(triangles: &[(u32, u32, u32)]) -> u32 {
    let mut count = 0;
    for triangle_tuple in triangles {
        let (a, b, c) = *triangle_tuple;
        if is_triangle(a, b, c) {
            count += 1;
        }
    }
    count
}

fn is_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_part1_one_line() {
        let input = "5 10 15\n";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input_part1(input).unwrap());
    }

    #[test]
    fn test_parse_input_part1_one_line_no_trailing_newline() {
        let input = "5 10 15";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input_part1(input).unwrap());
    }

    #[test]
    fn test_parse_input_part1_multiple_lines() {
        let input = "5 10 15\n1 2 3\n";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input_part1(input).unwrap());
    }

    #[test]
    fn test_parse_input_part1_multiple_lines_no_trailing_newline() {
        let input = "5 10 15\n1 2 3";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input_part1(input).unwrap());
    }

    #[test]
    fn test_parse_input_part2_one_triangle() {
        let input = "5\n10\n15\n";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input_part2(input).unwrap());
    }

    #[test]
    fn test_parse_input_part2_one_triangle_no_trailing_newline() {
        let input = "5\n10\n15";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input_part2(input).unwrap());
    }

    #[test]
    fn test_parse_input_part2_multiple_multiple_triangles() {
        let input1 = "5\n10\n15\n1\n2\n3\n";
        let input2 = " 5 1\n 10 2\n 15 3\n";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input_part2(input1).unwrap());
        assert_eq!(expected, parse_input_part2(input2).unwrap());
    }

    #[test]
    fn test_parse_input_part2_multiple_multiple_triangles_no_trailing_newline() {
        let input1 = "5\n10\n15\n1\n2\n3\n";
        let input2 = " 5 1\n 10 2\n 15 3\n";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input_part2(input1).unwrap());
        assert_eq!(expected, parse_input_part2(input2).unwrap());
    }

    #[test]
    fn test_transpose_1x1() {
        let matrix = vec![vec![1]];
        let expected = vec![vec![1]];
        assert_eq!(expected, transpose(matrix));
    }

    #[test]
    fn test_transpose_1x2() {
        let matrix = vec![vec![1, 2]];
        let expected = vec![vec![1], vec![2]];
        assert_eq!(expected, transpose(matrix));
    }

    #[test]
    fn test_transpose_2x1() {
        let matrix = vec![vec![1], vec![2]];
        let expected = vec![vec![1, 2]];
        assert_eq!(expected, transpose(matrix));
    }

    #[test]
    fn test_transpose_2x2() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 3], vec![2, 4]];
        assert_eq!(expected, transpose(matrix));
    }

    #[test]
    fn test_transpose_double_1x1() {
        let matrix = vec![vec![1]];
        let expected = vec![vec![1]];
        assert_eq!(expected, transpose(transpose(matrix)));
    }

    #[test]
    fn test_transpose_double_1x2() {
        let matrix = vec![vec![1, 2]];
        let expected = vec![vec![1, 2]];
        assert_eq!(expected, transpose(transpose(matrix)));
    }

    #[test]
    fn test_transpose_double_2x1() {
        let matrix = vec![vec![1], vec![2]];
        let expected = vec![vec![1], vec![2]];
        assert_eq!(expected, transpose(transpose(matrix)));
    }

    #[test]
    fn test_transpose_double_2x2() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(expected, transpose(transpose(matrix)));
    }
}
