// Amicable numbers
//
// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable
// numbers.
//
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The
// proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//
// Evaluate the sum of all the amicable numbers under 10000.

type Number = u32;

#[test]
fn test() {
  let limit: Number = 284;
  let primes: Vec<Number> = primes_up_to(limit);
  let mut prime_powers: Vec<Number> = vec![0; primes.len()];
  let mut number: Number = 1;
  let (mut hit_220, mut hit_284) = (false, false);
  loop {
    match next_prime_power_variant(number, &mut prime_powers, &primes, limit) {
      Some(next_number) => {
        number = next_number;
        if number == 220 {
          assert_eq!(hit_220, false);
          assert_eq!(
            proper_divisors(number, &prime_powers, &primes).iter().sum::<Number>(),
            284
          );
          hit_220 = true;
        } else if number == 284 {
          assert_eq!(hit_284, false);
          assert_eq!(
            proper_divisors(number, &prime_powers, &primes).iter().sum::<Number>(),
            220
          );
          hit_284 = true;
        }
      }
      None => break,
    }
  }
  assert!(hit_220);
  assert!(hit_284);
}

pub fn solve() -> Number {
  let limit: Number = 10_000;
  let primes: Vec<Number> = primes_up_to(limit);
  let mut prime_powers: Vec<Number> = vec![0; primes.len()];
  let mut number: Number = 1;

  let mut divisor_sums: Vec<Number> = vec![0; (limit + 1) as usize];
  let mut amicable_number_sum: Number = 0;
  loop {
    match next_prime_power_variant(number, &mut prime_powers, &primes, limit) {
      None => return amicable_number_sum,
      Some(next_number) => {
        number = next_number;
        let divisor_sum: Number = proper_divisors(number, &prime_powers, &primes).iter().sum();
        if divisor_sum != number && divisor_sum <= limit {
          divisor_sums[number as usize] = divisor_sum;
          if divisor_sums[divisor_sum as usize] == number {
            amicable_number_sum += number + divisor_sum;
          }
        }
      }
    }
  }
}

fn primes_up_to(limit: Number) -> Vec<Number> {
  let mut sieve: Vec<bool> = vec![true; (limit - 1) as usize];
  for index in 0..((limit - 1) as usize) {
    if sieve[index] {
      let prime = index + 2;
      let mut n = prime * 2;
      while n - 1 < limit as usize {
        sieve[n - 2] = false;
        n += prime;
      }
    }
  }
  sieve
    .iter()
    .enumerate()
    .flat_map(|(index, &is_prime)| if is_prime { Some((index + 2) as Number) } else { None })
    .collect()
}

fn next_prime_power_variant(
  number: Number,
  prime_powers: &mut Vec<Number>,
  primes: &Vec<Number>,
  number_limit: Number,
) -> Option<Number> {
  let mut next_number: Number = number;
  let mut prime_index: usize = 0;
  loop {
    if prime_index == primes.len() {
      return None;
    }
    prime_powers[prime_index] += 1;
    next_number *= primes[prime_index];
    if next_number <= number_limit {
      return Some(next_number);
    }
    next_number /= primes[prime_index].pow(prime_powers[prime_index] as u32);
    prime_powers[prime_index] = 0;
    prime_index += 1;
  }
}

fn proper_divisors(number: Number, prime_powers: &Vec<Number>, primes: &Vec<Number>) -> Vec<Number> {
  let unique_prime_count = prime_powers.iter().filter(|&&p| p != 0).count();
  let mut divisor_powers: Vec<Number> = vec![0; unique_prime_count];
  let mut divisor_primes: Vec<Number> = vec![0; unique_prime_count];

  let mut divisor_index: usize = 0;
  for index in 0..primes.len() {
    if prime_powers[index] != 0 {
      divisor_primes[divisor_index] = primes[index];
      divisor_index += 1;
    }
  }

  let mut divisors: Vec<Number> = vec![1];
  let mut divisor: Number = 1;
  loop {
    match next_prime_power_variant(divisor, &mut divisor_powers, &divisor_primes, number) {
      None => return divisors,
      Some(next_divisor) => {
        divisor = next_divisor;
        if divisor < number && number % divisor == 0 {
          divisors.push(divisor);
        }
      }
    }
  }
}
