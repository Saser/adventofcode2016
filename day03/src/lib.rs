extern crate base;
use base::{Part, ProblemSolver};

extern crate regex;
use regex::Regex;

use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day03)
}

struct Day03;

impl ProblemSolver for Day03 {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        let triangles = parse_input(input)?;
        match part {
            Part::One => Ok(count_triangles(&triangles).to_string()),
            Part::Two => Err("not yet implemented: part 2".to_owned()),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32, u32)>, String> {
    let lines = input.split_terminator('\n');
    base::utils::any_err(lines.map(parse_line))
}

fn parse_line(line: &str) -> Result<(u32, u32, u32), String> {
    let line = line.trim();
    let re = Regex::new(r"^\s*(?P<a>\d+)\s+(?P<b>\d+)\s+(?P<c>\d+)$").unwrap();
    let captures = re.captures(line).ok_or(format!("line contains invalid sides: {}", line))?;

    let a_str = captures.name("a").unwrap();
    let a = u32::from_str(a_str).unwrap();

    let b_str = captures.name("b").unwrap();
    let b = u32::from_str(b_str).unwrap();

    let c_str = captures.name("c").unwrap();
    let c = u32::from_str(c_str).unwrap();

    Ok((a, b, c))
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
    fn test_parse_input_one_line() {
        let input = "5 10 15\n";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_one_line_no_trailing_newline() {
        let input = "5 10 15";
        let expected = vec![(5, 10, 15)];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_multiple_lines() {
        let input = "5 10 15\n1 2 3\n";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_input_multiple_lines_no_trailing_newline() {
        let input = "5 10 15\n1 2 3";
        let expected = vec![(5, 10, 15), (1, 2, 3)];
        assert_eq!(expected, parse_input(input).unwrap());
    }
}
