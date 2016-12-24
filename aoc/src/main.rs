#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate base;
use base::{Part, ProblemSolver};

extern crate day01;
extern crate day02;

macro_rules! err_println {
    ( $( $arg : tt )* ) => {{
        use std::io::Write;
        let _ = writeln!(&mut std::io::stderr(), $($arg)*);
    }}
}

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
        Input::Literal(matches.value_of("input").unwrap().to_string())
    } else {
        Input::File(matches.value_of("file").unwrap().to_string())
    };

    let solver = get_solver(day).unwrap();
    let solution = match input {
        Input::Literal(literal) => solver.solve(&literal, &part),
        Input::File(file_path) => solver.solve_file(&file_path, &part),
    };

    match solution {
        Ok(answer) => println!("{}", answer),
        Err(error) => err_println!("Error: {}", error),
    }
}

fn get_solver(day: u32) -> Result<Box<ProblemSolver>, String> {
    match day {
        1 => Ok(day01::get_solver()),
        2 => Ok(day02::get_solver()),
        _ => Err("day either invalid or not implemented yet".to_string()),
    }
}
