use std::process::exit;
use std::time::{Duration, Instant};

use Problems::All;
use RunMode::Check;
use RunMode::Solve;

use crate::cli_args::Problems::ProblemNumbers;
use crate::cli_args::{Problems, RunConfig, RunMode};
use crate::spoilers::solution;
use crate::ProblemNumber;

const PROBLEM_NUMBER_UPPER_BOUND: ProblemNumber = 1024;

pub fn args_parse_error(msg: String) {
  eprintln!("Error: {}", msg);
  println!();
  crate::cli_args::print_usage();
  exit(1);
}

pub fn run(config: RunConfig) {
  match (config.mode, config.problems) {
    (Check, All) => check_all_problems(),
    (Check, ProblemNumbers(problem_numbers)) => check_problems(problem_numbers),
    (Solve, All) => solve_all_problems(),
    (Solve, ProblemNumbers(problem_numbers)) => solve_problems(problem_numbers),
  }
}

fn check_problems(problem_numbers: Vec<ProblemNumber>) {
  for problem_number in problem_numbers {
    let message: String = match calculate(problem_number) {
      Some(calculated) => fmt_check_result_or_unknown(calculated, solution(problem_number)),
      None => fmt_check_not_implemented(problem_number),
    };
    println!("{}", message)
  }
}

fn check_all_problems() {
  (1..=PROBLEM_NUMBER_UPPER_BOUND)
    .flat_map(calculate)
    .for_each(|calculated| {
      let actual_solution_maybe: Option<String> = solution(calculated.problem_number);
      println!("{}", fmt_check_result_or_unknown(calculated, actual_solution_maybe))
    });
}

fn solve_problems(problem_numbers: Vec<ProblemNumber>) {
  for problem_number in problem_numbers {
    match calculate(problem_number) {
      Some(calculated) => println!(
        "{} {:?}",
        fmt_solution(problem_number, calculated.solution),
        calculated.duration
      ),
      None => println!("{}", fmt_solution(problem_number, "[not implemented]".to_string())),
    }
  }
}

fn solve_all_problems() {
  (1..=PROBLEM_NUMBER_UPPER_BOUND)
    .flat_map(calculate)
    .for_each(|calculated| {
      println!(
        "{} ({:?})",
        fmt_solution(calculated.problem_number, calculated.solution),
        calculated.duration
      )
    });
}

struct Calculated {
  problem_number: ProblemNumber,
  solution: String,
  duration: Duration,
}

fn calculate(problem_number: ProblemNumber) -> Option<Calculated> {
  let time: Instant = Instant::now();
  crate::problems::solve_problem(problem_number).map(|solution| Calculated {
    problem_number,
    solution,
    duration: time.elapsed(),
  })
}

fn fmt_check_result(calculated: Calculated, actual_solution: String) -> String {
  if calculated.solution == actual_solution {
    fmt_check_success(calculated.problem_number, calculated.duration)
  } else {
    fmt_check_failure(calculated.problem_number, calculated.solution, calculated.duration)
  }
}

fn fmt_check_not_implemented(problem_number: ProblemNumber) -> String {
  fmt_solution(problem_number, "❓ [not implemented]".to_string())
}

fn fmt_check_result_or_unknown(calculated: Calculated, actual_solution_maybe: Option<String>) -> String {
  match actual_solution_maybe {
    Some(actual_solution) => fmt_check_result(calculated, actual_solution),
    None => fmt_check_unknown(calculated.problem_number, calculated.solution, calculated.duration),
  }
}

fn fmt_check_unknown(problem_number: ProblemNumber, calculated_solution: String, duration: Duration) -> String {
  fmt_solution(
    problem_number,
    format!(
      "❓ [actual solution unknown, calculated: {}] ({:?})",
      calculated_solution, duration
    ),
  )
}

fn fmt_check_success(problem_number: ProblemNumber, duration: Duration) -> String {
  fmt_solution(problem_number, format!("✅ ({:?})", duration))
}

fn fmt_check_failure(problem_number: ProblemNumber, wrong_solution: String, duration: Duration) -> String {
  fmt_solution(
    problem_number,
    format!("❌ [calculated wrong solution: {}] ({:?})", wrong_solution, duration),
  )
}

fn fmt_solution(problem_number: ProblemNumber, solution: String) -> String {
  format!("Problem {}: {}", fmt_problem_number(problem_number), solution)
}

fn fmt_problem_number(problem_number: ProblemNumber) -> String {
  format!("{:-3}", problem_number)
}
