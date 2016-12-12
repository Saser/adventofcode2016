#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    println!("Value of day: {}", matches.value_of("day").unwrap());
    println!("Value of part: {}", matches.value_of("part").unwrap());
    println!("Value of file: {}", matches.value_of("file").unwrap());
}
