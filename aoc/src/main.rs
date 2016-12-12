#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate base;
use base::{Part, ProblemSolver};

extern crate day01;

enum Input {
    Literal(String),
    File(String),
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let day = value_t!(matches, "day", u32).unwrap();
    let part = value_t!(matches, "part", base::Part).unwrap();
    let input = if matches.is_present("input") {
        Input::Literal(matches.value_of("input").unwrap().to_owned())
    } else {
        Input::File(matches.value_of("file").unwrap().to_owned())
    };

    let solver = get_solver(day).unwrap();
    let solution = match input {
        Input::Literal(literal) => solver.solve(&literal, &part),
        Input::File(file_path) => solver.solve_file(&file_path, &part),
    };
    println!("{}", solution);
    println!("solved problem for day {}", day);
}

fn get_solver(day: u32) -> Result<Box<ProblemSolver>, String> {
    match day {
        1 => Ok(day01::get_solver()),
        _ => Err("day either invalid or not implemented yet".to_owned()),
    }
}
