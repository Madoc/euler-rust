// Number letter counts
//
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19
// letters used in total.
//
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one
// hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British
// usage.

#[test]
fn test() {
  assert_eq!((1..=5).map(ones).sum::<u16>(), 19);
  assert_eq!(thousands(342), 23);
  assert_eq!(thousands(115), 20);
}

pub fn solve() -> u16 {
  (1..=1_000).map(thousands).sum::<u16>()
}

fn ones(n: u16) -> u16 {
  match n {
    0 => 0,
    3 | 7 | 8 => 5,
    4 | 5 | 9 => 4,
    _ => 3,
  }
}

fn tens(n: u16) -> u16 {
  match n {
    0..=9 => ones(n),
    10 => 3,
    40 | 50 | 60 => 5,
    11 | 12 | 20 | 30 | 80 | 90 => 6,
    15 | 16 | 70 => 7,
    13 | 14 | 18 | 19 => 8,
    17 => 9,
    21..=39 | 81..=89 | 91..=99 => 6 + ones(n % 10),
    71..=79 => 7 + ones(n % 10),
    _ => 5 + ones(n % 10),
  }
}

fn hundreds(n: u16) -> u16 {
  if n < 100 {
    tens(n)
  } else {
    ones(n / 100)
      + 7
      + match n % 100 {
        0 => 0,
        t => tens(t) + 3,
      }
  }
}

fn thousands(n: u16) -> u16 {
  if n < 1_000 {
    hundreds(n)
  } else {
    thousands(n / 1_000)
      + 8
      + match n % 1_000 {
        0 => 0,
        t => hundreds(t),
      }
  }
}
