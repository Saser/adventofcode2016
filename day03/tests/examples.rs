extern crate base;
use base::Part;

extern crate day03;

use std::str::FromStr;

fn get_answer(input: &str, part: Part) -> u32 {
    let solver = day03::get_solver();
    let solution_str = &solver.solve(input, part).unwrap();
    u32::from_str(solution_str).unwrap()
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "5 10 25";
        assert_eq!(get_answer(input, Part::One), 0);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 \
                     603\n";
        assert_eq!(get_answer(input, Part::Two), 6);
    }
}
