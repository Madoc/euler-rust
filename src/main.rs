mod cli_args;
mod problems;
mod run;
mod spoilers;

pub type ProblemNumber = u16;

fn main() {
  match cli_args::parse_args() {
    Err(msg) => run::args_parse_error(msg),
    Ok(config) => run::run(config),
  }
}
