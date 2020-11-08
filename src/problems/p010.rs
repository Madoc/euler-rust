// Summation of primes
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

#[test]
fn test() {
  assert_eq!(sum_of_primes_below(10), 17)
}

pub fn solve() -> u64 {
  sum_of_primes_below(2_000_000)
}

fn sum_of_primes_below(limit: usize) -> u64 {
  let mut prime_sieve: Vec<bool> = vec![true; limit];
  prime_sieve[0] = false;
  prime_sieve[1] = false;
  for i in 2..limit {
    if prime_sieve[i] {
      for m in (i * 2..limit).step_by(i) {
        prime_sieve[m] = false
      }
    }
  }
  let mut sum: u64 = 0;
  for (num, is_prime) in prime_sieve.iter().enumerate() {
    if *is_prime {
      sum += num as u64
    }
  }
  sum
}
