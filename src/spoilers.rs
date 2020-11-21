// !!!!!!!!!!!!!!!!!!!!!!!
// !!! SPOILER WARNING !!!
// !!!!!!!!!!!!!!!!!!!!!!!
//
// This file contains the solutions for the Project Euler problems.

use crate::ProblemNumber;

pub fn solution(problem_number: ProblemNumber) -> Option<String> {
  match problem_number {
    001 => Some("233168"),
    002 => Some("4613732"),
    003 => Some("6857"),
    004 => Some("906609"),
    005 => Some("232792560"),
    006 => Some("25164150"),
    007 => Some("104743"),
    008 => Some("23514624000"),
    009 => Some("31875000"),
    010 => Some("142913828922"),
    011 => Some("70600674"),
    012 => Some("76576500"),
    013 => Some("5537376230"),
    014 => Some("837799"),
    015 => Some("137846528820"),
    016 => Some("1366"),
    017 => Some("21124"),
    018 => Some("1074"),
    019 => Some("171"),
    _ => None,
  }
  .map(|str| str.to_string())
}
