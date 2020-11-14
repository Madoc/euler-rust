// Smallest multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use std::cmp::max;
use std::collections::HashMap;

#[test]
fn test() {
  assert_eq!(smallest_divisible_number(10), 2520)
}

pub fn solve() -> u64 {
  smallest_divisible_number(20)
}

#[allow(unused_assignments)] // Maybe a compiler error?
fn smallest_divisible_number(max_divisor: u64) -> u64 {
  let unique_divisors: Vec<u64> = {
    let mut divisor_candidates: Vec<u64> = (2..=max_divisor).collect();
    'next_divisor: for divisor_index in 0..divisor_candidates.len() - 1 {
      for greater_divisor_index in (divisor_index + 1)..=divisor_candidates.len() - 1 {
        if divisor_candidates[greater_divisor_index] % divisor_candidates[divisor_index] == 0 {
          divisor_candidates[divisor_index] = 0;
          continue 'next_divisor;
        }
      }
    }
    divisor_candidates.retain(|&d| d != 0);
    divisor_candidates
  };

  let mut prime_powers: HashMap<u64, u64> = HashMap::new();
  for &divisor in &unique_divisors {
    let mut remaining: u64 = divisor;
    let mut divisor_prime_powers: HashMap<u64, u64> = HashMap::new();
    while remaining > 1 {
      let mut min_prime: u64 = 2;
      'next_prime: for prime_candidate in min_prime..=remaining {
        if remaining % prime_candidate == 0 {
          remaining /= prime_candidate;
          match divisor_prime_powers.get(&prime_candidate) {
            None => divisor_prime_powers.insert(prime_candidate, 1),
            Some(&already) => divisor_prime_powers.insert(prime_candidate, already + 1),
          };
          min_prime = prime_candidate;
          continue 'next_prime;
        }
      }
    }
    for (prime, power) in divisor_prime_powers {
      match prime_powers.get(&prime) {
        None => prime_powers.insert(prime, power),
        Some(&already) => prime_powers.insert(prime, max(power, already)),
      };
    }
  }

  prime_powers.iter().fold(1u64, |n, (&p, &c)| n * p.pow(c as u32))
}
