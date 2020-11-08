// Largest prime factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

#[test]
fn test() {
  assert_eq!(largest_prime_factor(13_195), 29)
}

pub fn solve() -> u64 {
  largest_prime_factor(600_851_475_143)
}

fn largest_prime_factor(number: u64) -> u64 {
  let mut remaining_number: u64 = number;
  let mut largest_prime_factor: u64 = 1;
  let mut prime_candidate: u64 = 2;
  while prime_candidate <= remaining_number {
    if remaining_number % prime_candidate == 0 {
      remaining_number = remaining_number / prime_candidate;
      largest_prime_factor = prime_candidate;
    } else {
      prime_candidate += 1;
    }
  }
  largest_prime_factor
}
