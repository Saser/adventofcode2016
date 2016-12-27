extern crate base;
use base::{Part, ProblemSolver};

extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<name>[a-z]+(-[a-z]+)*?)-(?P<sid_chk>\d+\[[a-z]{5}\])$").unwrap();
        }

        let captures = RE.captures(s).ok_or(format!("Room::from_str: invalid room string: {}", s))?;

        let name = remove_dashes(captures.name("name").unwrap());
        let (sector_id, checksum) = sector_id_and_checksum(captures.name("sid_chk").unwrap())?;
        Ok(Room {
            name: name,
            sector_id: sector_id,
            checksum: checksum,
        })
    }
}

impl Room {
    fn is_real(&self) -> bool {
        unimplemented!()
    }
}

fn calculate_checksum(s: &str) -> String {
    unimplemented!()
}

fn remove_dashes(s: &str) -> String {
    s.split('-').collect::<String>()
}

fn sector_id_and_checksum(s: &str) -> Result<(u32, String), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<sector_id>\d+)\[(?P<checksum>[a-z]{5})\]$").unwrap();
    }

    let captures = RE.captures(s).ok_or(format!("sector_id_and_checksum: invalid string: {}", s))?;

    let sector_id_str = captures.name("sector_id").unwrap();
    let sector_id = u32::from_str(sector_id_str).unwrap();

    let checksum_str = captures.name("checksum").unwrap();

    Ok((sector_id, checksum_str.to_owned()))
}

fn char_frequencies(s: &str) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();
    for c in s.chars() {
        let char_frequency = frequencies.entry(c).or_insert(0);
        *char_frequency += 1;
    }
    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_parse_room {
        use super::*;

        #[test]
        fn example1() {
            let room_str = "aaaaa-bbb-z-y-x-123[abxyz]";
            let room = Room::from_str(room_str).unwrap();
            assert_eq!("aaaaabbbzyx".to_owned(), room.name);
            assert_eq!(123, room.sector_id);
            assert_eq!("abxyz".to_owned(), room.checksum);
        }

        #[test]
        fn example2() {
            let room_str = "a-b-c-d-e-f-g-h-987[abcde]";
            let room = Room::from_str(room_str).unwrap();
            assert_eq!("abcdefgh".to_owned(), room.name);
            assert_eq!(987, room.sector_id);
            assert_eq!("abcde".to_owned(), room.checksum);
        }

        #[test]
        fn example3() {
            let room_str = "not-a-real-room-404[oarel]";
            let room = Room::from_str(room_str).unwrap();
            assert_eq!("notarealroom".to_owned(), room.name);
            assert_eq!(404, room.sector_id);
            assert_eq!("oarel".to_owned(), room.checksum);
        }

        #[test]
        fn example4() {
            let room_str = "totally-real-room-200[decoy]";
            let room = Room::from_str(room_str).unwrap();
            assert_eq!("totallyrealroom".to_owned(), room.name);
            assert_eq!(200, room.sector_id);
            assert_eq!("decoy".to_owned(), room.checksum);
        }
    }

    mod test_room {
        use super::*;

        #[test]
        fn is_real_example1() {
            let room = Room {
                name: "aaaaabbbzyx".to_owned(),
                sector_id: 123,
                checksum: "abxyz".to_owned(),
            };
            assert!(room.is_real());
        }

        #[test]
        fn is_real_example2() {
            let room = Room {
                name: "abcdefgh".to_owned(),
                sector_id: 987,
                checksum: "abcde".to_owned(),
            };
            assert!(room.is_real());
        }

        #[test]
        fn is_real_example3() {
            let room = Room {
                name: "notarealroom".to_owned(),
                sector_id: 404,
                checksum: "oarel".to_owned(),
            };
            assert!(room.is_real());
        }

        #[test]
        fn is_real_example4() {
            let room = Room {
                name: "totallyrealroom".to_owned(),
                sector_id: 200,
                checksum: "decoy".to_owned(),
            };
            assert!(!room.is_real());
        }
    }

    mod test_remove_dashes {
        use super::*;

        #[test]
        fn simple() {
            assert_eq!("abc".to_owned(), remove_dashes("a-b-c"));
        }

        #[test]
        fn leading_dash() {
            assert_eq!("abc".to_owned(), remove_dashes("-a-b-c"));
        }

        #[test]
        fn several_leading_dashes() {
            assert_eq!("abc".to_owned(), remove_dashes("----a-b-c"));
        }

        #[test]
        fn trailing_dash() {
            assert_eq!("abc".to_owned(), remove_dashes("a-b-c-"));
        }

        #[test]
        fn several_trailing_dashes() {
            assert_eq!("abc".to_owned(), remove_dashes("a-b-c----"));
        }

        #[test]
        fn several_inner_dashes() {
            assert_eq!("abc".to_owned(), remove_dashes("a---b---c"));
        }

        #[test]
        fn empty() {
            assert_eq!("".to_owned(), remove_dashes(""));
        }

        #[test]
        fn only_dashes() {
            assert_eq!("".to_owned(), remove_dashes("----"));
        }
    }

    mod test_sector_id_and_checksum {
        use super::*;

        #[test]
        fn simple() {
            let (sector_id, checksum) = sector_id_and_checksum("123[abcde]").unwrap();
            assert_eq!(123, sector_id);
            assert_eq!("abcde".to_owned(), checksum);
        }

        #[test]
        fn too_short_checksum() {
            let err = sector_id_and_checksum("123[abe]");
            assert!(err.is_err());
        }

        #[test]
        fn empty_sector_id() {
            let err = sector_id_and_checksum("[abcde]");
            assert!(err.is_err());
        }

        #[test]
        fn negative_sector_id() {
            let err = sector_id_and_checksum("-123[abcde]");
            assert!(err.is_err());
        }

        #[test]
        fn sector_id_with_letters() {
            let err = sector_id_and_checksum("123asd[abcde]");
            assert!(err.is_err());
        }

        #[test]
        fn wrong_parentheses() {
            let err_strs = ["123(abcde)", "123{abcde}", "123<abcde>"];
            for err_str in &err_strs {
                let err = sector_id_and_checksum(err_str);
                assert!(err.is_err());
            }
        }

        #[test]
        fn trailing_characters() {
            let err = sector_id_and_checksum("123[abcde]herp");
            assert!(err.is_err());
        }
    }

    mod test_char_frequencies {
        use super::*;

        #[test]
        fn single_char() {
            let s = "aaaaa";
            let frequencies = char_frequencies(s);
            assert_eq!(&5, frequencies.get(&'a').unwrap());
        }

        #[test]
        fn sorted_str_multi_char_different_frequencies() {
            let s = "aaabbc";
            let frequencies = char_frequencies(s);
            assert_eq!(&3, frequencies.get(&'a').unwrap());
            assert_eq!(&2, frequencies.get(&'b').unwrap());
            assert_eq!(&1, frequencies.get(&'c').unwrap());
        }

        #[test]
        fn unsorted_str_multi_char_different_frequencies() {
            let s = "abbaca";
            let frequencies = char_frequencies(s);
            assert_eq!(&3, frequencies.get(&'a').unwrap());
            assert_eq!(&2, frequencies.get(&'b').unwrap());
            assert_eq!(&1, frequencies.get(&'c').unwrap());
        }

        #[test]
        fn unsorted_str_multi_char_same_frequency() {
            let s = "acbccbbaa";
            let frequencies = char_frequencies(s);
            assert_eq!(&3, frequencies.get(&'a').unwrap());
            assert_eq!(&3, frequencies.get(&'b').unwrap());
            assert_eq!(&3, frequencies.get(&'c').unwrap());
        }

        #[test]
        fn empty() {
            let s = "";
            let frequencies = char_frequencies(s);
            assert!(frequencies.is_empty());
        }
    }

    mod test_calculate_checksum {
        use super::*;

        #[test]
        fn sorted_str_same_frequencies() {
            let s = "aabbccddee";
            assert_eq!("abcde".to_owned(), calculate_checksum(s));
        }

        #[test]
        fn unsorted_str_same_frequencies() {
            let s = "ecbcadbade";
            assert_eq!("abcde".to_owned(), calculate_checksum(s));
        }

        #[test]
        fn sorted_str_different_frequencies() {
            let s = "abbcccddddeeeee";
            assert_eq!("edcba".to_owned(), calculate_checksum(s));
        }

        #[test]
        fn unsorted_str_different_frequencies() {
            let s = "bcecadebdcdedee";
            assert_eq!("edcba".to_owned(), calculate_checksum(s));
        }

        #[test]
        fn sorted_str_tiebreaker() {
            let s = "aaaabbbccddee";
            assert_eq!("abcde".to_owned(), calculate_checksum(s));
        }

        #[test]
        fn unsorted_str_tiebreaker() {
            let s = "abaecdabbdace";
            assert_eq!("abcde".to_owned(), calculate_checksum(s));
        }
    }
}
