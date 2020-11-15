use std::process::exit;

use Problems::All;
use RunMode::Check;
use RunMode::Solve;

use crate::cli_args::Problems::ProblemNumbers;
use crate::cli_args::{Problems, RunConfig, RunMode};
use crate::problems::solve_problem;
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
    let message: String = match solve_problem(problem_number) {
      Some(calculated_solution) => {
        fmt_check_result_or_unknown(problem_number, calculated_solution, solution(problem_number))
      }
      None => fmt_check_not_implemented(problem_number),
    };
    println!("{}", message)
  }
}

fn check_all_problems() {
  for problem_number in 1..=PROBLEM_NUMBER_UPPER_BOUND {
    match solve_problem(problem_number) {
      Some(calculated_solution) => println!(
        "{}",
        fmt_check_result_or_unknown(problem_number, calculated_solution, solution(problem_number))
      ),
      None => (),
    }
  }
}

fn solve_problems(problem_numbers: Vec<ProblemNumber>) {
  for problem_number in problem_numbers {
    match solve_problem(problem_number) {
      Some(solution) => println!("{}", fmt_solution(problem_number, solution)),
      None => println!("{}", fmt_solution(problem_number, "[not implemented]".to_string())),
    }
  }
}

fn solve_all_problems() {
  for problem_number in 1..=PROBLEM_NUMBER_UPPER_BOUND {
    match solve_problem(problem_number) {
      Some(solution) => println!("{}", fmt_solution(problem_number, solution)),
      None => (),
    }
  }
}

fn fmt_check_result(problem_number: ProblemNumber, calculated_solution: String, actual_solution: String) -> String {
  if calculated_solution == actual_solution {
    fmt_check_success(problem_number)
  } else {
    fmt_check_failure(problem_number, calculated_solution)
  }
}

fn fmt_check_not_implemented(problem_number: ProblemNumber) -> String {
  fmt_solution(problem_number, "❓ [not implemented]".to_string())
}

fn fmt_check_result_or_unknown(
  problem_number: ProblemNumber,
  calculated_solution: String,
  actual_solution_maybe: Option<String>,
) -> String {
  match actual_solution_maybe {
    Some(actual_solution) => fmt_check_result(problem_number, calculated_solution, actual_solution),
    None => fmt_check_unknown(problem_number, calculated_solution),
  }
}

fn fmt_check_unknown(problem_number: ProblemNumber, calculated_solution: String) -> String {
  fmt_solution(
    problem_number,
    format!("❓ [actual solution unknown, calculated: {}]", calculated_solution),
  )
}

fn fmt_check_success(problem_number: ProblemNumber) -> String {
  fmt_solution(problem_number, "✅".to_string())
}

fn fmt_check_failure(problem_number: ProblemNumber, wrong_solution: String) -> String {
  fmt_solution(
    problem_number,
    format!("❌ [calculated wrong solution: {}]", wrong_solution),
  )
}

fn fmt_solution(problem_number: ProblemNumber, solution: String) -> String {
  format!("Problem {}: {}", fmt_problem_number(problem_number), solution)
}

fn fmt_problem_number(problem_number: ProblemNumber) -> String {
  format!("{:-3}", problem_number)
}
