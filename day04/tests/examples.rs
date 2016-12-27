extern crate base;
use base::Part;

extern crate day04;

use std::str::FromStr;

fn get_answer(input: &str, part: Part) -> u32 {
    let solver = day04::get_solver();
    let solution_str = &solver.solve(input, part).unwrap();
    u32::from_str(solution_str).unwrap()
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]\n";
        assert_eq!(get_answer(input, Part::One), 1514);
    }
}
