extern crate base;
use base::{Part, ProblemSolver};

use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day04)
}

struct Day04;

impl ProblemSolver for Day04 {
    fn solve(&self, input: &str, part: Part) -> Result<String, String> {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err("herp".to_owned())
    }
}

impl Room {
    fn is_real(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_room_example1() {
        let room_str = "aaaaa-bbb-z-y-x-123[abxyz]";
        let room = Room::from_str(room_str).unwrap();
        assert_eq!("aaaaabbbzyx".to_owned(), room.name);
        assert_eq!(123, room.sector_id);
        assert_eq!("abxyz".to_owned(), room.name);
    }

    #[test]
    fn test_parse_room_example2() {
        let room_str = "a-b-c-d-e-f-g-h-987[abcde]";
        let room = Room::from_str(room_str).unwrap();
        assert_eq!("abcdefgh".to_owned(), room.name);
        assert_eq!(987, room.sector_id);
        assert_eq!("abcde".to_owned(), room.name);
    }

    #[test]
    fn test_parse_room_example3() {
        let room_str = "not-a-real-room-404[oarel]";
        let room = Room::from_str(room_str).unwrap();
        assert_eq!("notarealroom".to_owned(), room.name);
        assert_eq!(404, room.sector_id);
        assert_eq!("oarel".to_owned(), room.name);
    }

    #[test]
    fn test_parse_room_example4() {
        let room_str = "totally-real-room-200[decoy]";
        let room = Room::from_str(room_str).unwrap();
        assert_eq!("totallyrealroom".to_owned(), room.name);
        assert_eq!(200, room.sector_id);
        assert_eq!("decoy".to_owned(), room.name);
    }

    #[test]
    fn test_room_is_real_example1() {
        let room = Room {
            name: "aaaaabbbzyx".to_owned(),
            sector_id: 123,
            checksum: "abxyz".to_owned(),
        };
        assert!(room.is_real());
    }

    #[test]
    fn test_room_is_real_example2() {
        let room = Room {
            name: "abcdefgh".to_owned(),
            sector_id: 987,
            checksum: "abcde".to_owned(),
        };
        assert!(room.is_real());
    }

    #[test]
    fn test_room_is_real_example3() {
        let room = Room {
            name: "notarealroom".to_owned(),
            sector_id: 404,
            checksum: "oarel".to_owned(),
        };
        assert!(room.is_real());
    }

    #[test]
    fn test_room_is_real_example4() {
        let room = Room {
            name: "totallyrealroom".to_owned(),
            sector_id: 200,
            checksum: "decoy".to_owned(),
        };
        assert!(!room.is_real());
    }
}
