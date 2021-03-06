#[macro_use]
extern crate clap;
use clap::App;

extern crate base;
use base::ProblemSolver;

extern crate day01;
extern crate day02;
extern crate day03;
extern crate day04;

use std::time::Instant;

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

    let timer = Instant::now();

    let solver = get_solver(day).unwrap();
    let solution = match input {
        Input::Literal(literal) => solver.solve(&literal, part),
        Input::File(file_path) => {
            let input = base::utils::read_file_as_string(&file_path);
            solver.solve(&input, part)
        }
    };

    match solution {
        Ok(answer) => println!("{}", answer),
        Err(error) => err_println!("Error: {}", error),
    };

    let elapsed = timer.elapsed();
    println!("Time elapsed: {}.{:09} seconds",
             elapsed.as_secs(),
             elapsed.subsec_nanos());
}

fn get_solver(day: u32) -> Result<Box<ProblemSolver>, String> {
    match day {
        1 => Ok(day01::get_solver()),
        2 => Ok(day02::get_solver()),
        3 => Ok(day03::get_solver()),
        4 => Ok(day04::get_solver()),
        _ => Err("day either invalid or not implemented yet".to_string()),
    }
}
