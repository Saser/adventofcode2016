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

fn remove_dashes(s: &str) -> String {
    unimplemented!()
}

fn sector_id_and_checksum(s: &str) -> Result<(u32, String), String> {
    unimplemented!()
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

    #[test]
    fn test_remove_dashes() {
        assert_eq!("abc".to_owned(), remove_dashes("a-b-c"));
    }

    #[test]
    fn test_remove_dashes_leading_dash() {
        assert_eq!("abc".to_owned(), remove_dashes("-a-b-c"));
    }

    #[test]
    fn test_remove_dashes_several_leading_dashes() {
        assert_eq!("abc".to_owned(), remove_dashes("----a-b-c"));
    }

    #[test]
    fn test_remove_dashes_trailing_dash() {
        assert_eq!("abc".to_owned(), remove_dashes("a-b-c-"));
    }

    #[test]
    fn test_remove_dashes_several_trailing_dashes() {
        assert_eq!("abc".to_owned(), remove_dashes("a-b-c----"));
    }

    #[test]
    fn test_remove_dashes_several_inner_dashes() {
        assert_eq!("abc".to_owned(), remove_dashes("a---b---c"));
    }

    #[test]
    fn test_remove_dashes_empty() {
        assert_eq!("".to_owned(), remove_dashes(""));
    }

    #[test]
    fn test_remove_dashes_only_dashes() {
        assert_eq!("".to_owned(), remove_dashes("----"));
    }

    #[test]
    fn test_sector_id_and_checksum() {
        let (sector_id, checksum) = sector_id_and_checksum("123[abcde]").unwrap();
        assert_eq!(123, sector_id);
        assert_eq!("abcde".to_owned(), checksum);
    }

    #[test]
    fn test_sector_id_and_checksum_too_short_checksum() {
        let err = sector_id_and_checksum("123[abe]");
        assert!(err.is_err());
    }

    #[test]
    fn test_sector_id_and_checksum_empty_sector_id() {
        let err = sector_id_and_checksum("[abcde]");
        assert!(err.is_err());
    }

    #[test]
    fn test_sector_id_and_checksum_negative_sector_id() {
        let err = sector_id_and_checksum("-123[abcde]");
        assert!(err.is_err());
    }

    #[test]
    fn test_sector_id_and_checksum_sector_id_with_letters() {
        let err = sector_id_and_checksum("123asd[abcde]");
        assert!(err.is_err());
    }

    #[test]
    fn test_sector_id_and_checksum_wrong_parentheses() {
        let err_strs = ["123(abcde)", "123{abcde}", "123<abcde>"];
        for err_str in &err_strs {
            let err = sector_id_and_checksum(err_str);
            assert!(err.is_err());
        }
    }

    #[test]
    fn test_sector_id_and_checksum_trailing_characters() {
        let err = sector_id_and_checksum("123[abcde]herp");
        assert!(err.is_err());
    }
}
