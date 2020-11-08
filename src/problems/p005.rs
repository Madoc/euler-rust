// Smallest multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

#[test]
fn test() {
  assert_eq!(smallest_divisible_number(10), 2520)
}

// TODO too slow
pub fn solve() -> u64 {
  smallest_divisible_number(20)
}

fn smallest_divisible_number(max_divisor: u64) -> u64 {
  let upper_limit: u64 = {
    let mut n: u64 = 1;
    for i in 1..max_divisor {
      n *= i
    }
    n
  };
  'next_candidate: for candidate in 1..upper_limit {
    for div in 1..max_divisor {
      if candidate % div != 0 {
        continue 'next_candidate;
      }
    }
    return candidate;
  }
  0
}
