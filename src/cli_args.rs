use std::env::args;

use crate::cli_args::Problems::{All, ProblemNumbers};
use crate::ProblemNumber;

pub struct RunConfig {
  pub mode: RunMode,
  pub problems: Problems,
}

pub enum RunMode {
  Check,
  Solve,
}

pub enum Problems {
  All,
  ProblemNumbers(Vec<ProblemNumber>),
}

pub fn parse_args() -> Result<RunConfig, String> {
  let argv: Vec<String> = args().collect();
  let run_mode: RunMode = match argv.get(1).as_ref().map(|&str| str.as_str()) {
    Some("check") => RunMode::Check,
    Some("solve") => RunMode::Solve,
    Some(unknown) => return Result::Err(format!("Unknown command '{}'", unknown)),
    None => return Err("Expected COMMAND as first argument".to_string()),
  };

  let problem_args: Vec<String> = argv.iter().skip(2).map(|s| s.to_string()).collect();
  let mut is_all: bool = false;
  let mut problem_numbers: Vec<ProblemNumber> = Vec::new();
  for arg in problem_args {
    if arg == "all" {
      is_all = true;
    } else {
      match str::parse::<ProblemNumber>(&arg) {
        Ok(number) => problem_numbers.push(number),
        Err(_) => return Err(format!("Expected number or 'all' instead of '{}'", arg)),
      }
    }
  }

  if is_all {
    Ok(RunConfig {
      mode: run_mode,
      problems: All,
    })
  } else if problem_numbers.is_empty() {
    Err("Missing PROBLEMS".to_string())
  } else {
    problem_numbers.sort();
    problem_numbers.dedup();
    Ok(RunConfig {
      mode: run_mode,
      problems: ProblemNumbers(problem_numbers),
    })
  }
}

pub fn print_usage() {
  println!(
    "USAGE:
    {} COMMAND PROBLEMS

COMMAND:
    solve    Print the computed solutions to the problems
    check    Check solutions for correctness, but do not print them

PROBLEMS:
    all             All problems for which solutions are implemented
    <NUMBER> ...    Only problems with the given number(s)

For Project Euler, see: https://projecteuler.net/
",
    args().nth(0).unwrap_or("euler-rust".to_string())
  );
}
