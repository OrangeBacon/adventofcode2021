use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

mod day1;
mod day2;

const SOLVERS: &[fn(&str) -> Result<(String, String)>] = &[day1::day1, day2::day2];

#[derive(Debug)]
enum Errors {
    InvalidDay(String),
    InvalidFile(String, std::io::Error),
    SolverError(String, anyhow::Error),
}

fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("days")
                .takes_value(true)
                .multiple(true)
                .use_delimiter(true),
        )
        .get_matches();

    let mut errors = vec![];

    let days: Vec<String> = matches
        .values_of("days")
        .and_then(|val| Some(val.map(|v| v.to_string()).collect()))
        .unwrap_or_else(|| vec![SOLVERS.len().to_string()]);

    for day in days {
        let day_num = match day.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                errors.push(Errors::InvalidDay(day.to_string()));
                continue;
            }
        };

        let day_num = match day_num.checked_sub(1) {
            Some(num) => num,
            None => {
                errors.push(Errors::InvalidDay(day.to_string()));
                continue;
            }
        };

        let solver = match SOLVERS.get(day_num) {
            Some(solver) => solver,
            None => {
                errors.push(Errors::InvalidDay(day.to_string()));
                continue;
            }
        };

        let data = match std::fs::read_to_string(format!("data/day{}.txt", day)) {
            Ok(data) => data,
            Err(e) => {
                errors.push(Errors::InvalidFile(day.to_string(), e));
                continue;
            }
        };

        let result = solver(&data);

        match result {
            Ok((a, b)) => {
                println!("Day {} a = {}", day, a);
                println!("Day {} b = {}", day, b);
            }
            Err(e) => {
                errors.push(Errors::SolverError(day.to_string(), e));
            }
        }
    }

    for error in errors {
        eprintln!("Error: {:?}", error);
    }

    Ok(())
}
