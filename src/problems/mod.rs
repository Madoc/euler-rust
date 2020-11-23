use crate::ProblemNumber;

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
mod p012;
mod p013;
mod p014;
mod p015;
mod p016;
mod p017;
mod p018;
mod p019;
mod p020;
mod p021;
mod p022;

pub fn solve_problem(number: ProblemNumber) -> Option<String> {
  match number {
    001 => Some(p001::solve().to_string()),
    002 => Some(p002::solve().to_string()),
    003 => Some(p003::solve().to_string()),
    004 => Some(p004::solve().to_string()),
    005 => Some(p005::solve().to_string()),
    006 => Some(p006::solve().to_string()),
    007 => Some(p007::solve().to_string()),
    008 => Some(p008::solve().to_string()),
    009 => Some(p009::solve().to_string()),
    010 => Some(p010::solve().to_string()),
    011 => Some(p011::solve().to_string()),
    012 => Some(p012::solve().to_string()),
    013 => Some(p013::solve().to_string()),
    014 => Some(p014::solve().to_string()),
    015 => Some(p015::solve().to_string()),
    016 => Some(p016::solve().to_string()),
    017 => Some(p017::solve().to_string()),
    018 => Some(p018::solve().to_string()),
    019 => Some(p019::solve().to_string()),
    020 => Some(p020::solve().to_string()),
    021 => Some(p021::solve().to_string()),
    022 => Some(p022::solve().to_string()),
    _ => None,
  }
}
