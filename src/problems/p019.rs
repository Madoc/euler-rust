// Counting Sundays
//
// You are given the following information, but you may prefer to do some research for yourself.
//
// * 1 Jan 1900 was a Monday.
// * Thirty days has September,
//   April, June and November.
//   All the rest have thirty-one,
//   Saving February alone,
//   Which has twenty-eight, rain or shine.
//   And on leap years, twenty-nine.
// * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

// Implementation note: Abstained from the chrono crate for a faithful implementation.

use crate::problems::p019::Weekday::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY};

pub fn solve() -> u16 {
  let mut counted_sundays: u16 = 0;
  let mut day: Day = Day {
    day: 1,
    month: 1,
    year: 1900,
    weekday: MONDAY,
  };
  while day.year < 2001 {
    if day.year >= 1901 && day.year <= 2000 && day.weekday == SUNDAY {
      counted_sundays += 1;
    }
    day.increment_to_next_month_beginning();
  }
  counted_sundays
}

#[derive(PartialEq)]
enum Weekday {
  MONDAY,
  TUESDAY,
  WEDNESDAY,
  THURSDAY,
  FRIDAY,
  SATURDAY,
  SUNDAY,
}
impl Weekday {
  fn successor(week_day: &Weekday) -> Weekday {
    match week_day {
      MONDAY => TUESDAY,
      TUESDAY => WEDNESDAY,
      WEDNESDAY => THURSDAY,
      THURSDAY => FRIDAY,
      FRIDAY => SATURDAY,
      SATURDAY => SUNDAY,
      SUNDAY => MONDAY,
    }
  }
}

struct Day {
  day: u16,
  month: u16,
  year: u16,
  weekday: Weekday,
}
impl Day {
  fn increment_day(&mut self) {
    self.day += 1;
    self.weekday = Weekday::successor(&self.weekday);
    if self.day > 28 {
      let crossed_month_boundary: bool = match self.month {
        4 | 6 | 9 | 11 => self.day > 30,
        2 => {
          if (self.year % 4 == 0) && (self.year % 100 != 0 || self.year % 400 == 0) {
            self.day > 29
          } else {
            self.day > 28
          }
        }
        _ => self.day > 31,
      };
      if crossed_month_boundary {
        self.day = 1;
        self.month += 1;
        if self.month > 12 {
          self.month = 1;
          self.year += 1;
        }
      }
    }
  }

  fn increment_to_next_month_beginning(&mut self) {
    loop {
      self.increment_day();
      if self.day == 1 {
        break;
      }
    }
  }
}
