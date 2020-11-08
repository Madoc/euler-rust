mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;
mod p011;

pub fn solve_problem(number: u16) -> Option<String> {
  match number {
    01 => Some(p001::solve().to_string()),
    02 => Some(p002::solve().to_string()),
    03 => Some(p003::solve().to_string()),
    04 => Some(p004::solve().to_string()),
    05 => Some(p005::solve().to_string()),
    06 => Some(p006::solve().to_string()),
    07 => Some(p007::solve().to_string()),
    08 => Some(p008::solve().to_string()),
    09 => Some(p009::solve().to_string()),
    10 => Some(p010::solve().to_string()),
    11 => Some(p011::solve().to_string()),
    _ => None,
  }
}
