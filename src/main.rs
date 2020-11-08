mod problems;

const PROBLEM_NUMBER: u16 = 11;

fn main() {
  print!("\nProblem {}\n\n", PROBLEM_NUMBER);
  match problems::solve_problem(PROBLEM_NUMBER) {
    Some(solution) => print!("Solution: {}\n\n", solution),
    None => print!("No solution implemented.\n\n"),
  }
}
