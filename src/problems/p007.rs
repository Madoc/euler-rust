// 10001st prime
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?

#[test]
fn test() {
  assert_eq!(nth_prime(6), 13)
}

pub fn solve() -> u64 {
  nth_prime(10_001)
}

fn nth_prime(n: usize) -> u64 {
  let mut primes: Vec<u64> = Vec::new();
  let mut candidate: u64 = 3;
  let mut last_prime: u64 = 2;
  primes.push(2);
  'next_candidate: while primes.len() < n {
    for prime in &primes {
      if candidate % prime == 0 {
        candidate += 1;
        continue 'next_candidate;
      }
    }
    primes.push(candidate);
    last_prime = candidate;
    candidate += 2;
  }
  last_prime
}
