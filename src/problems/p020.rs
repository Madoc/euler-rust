// Factorial digit sum
//
// n! means n × (n − 1) × ... × 3 × 2 × 1
//
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//
// Find the sum of the digits in the number 100!

#[test]
fn test() {
  assert_eq!(fac_digit_sum(10), 27);
}

pub fn solve() -> u16 {
  fac_digit_sum(100)
}

fn fac_digit_sum(n: u16) -> u16 {
  let mut digits: Vec<u8> = vec![1];
  for f in 2..=n {
    digits = mul(digits, f);
  }
  digits.iter().map(|&d| d as u16).sum()
}

fn mul(digits: Vec<u8>, f: u16) -> Vec<u8> {
  let mut result: Vec<u8> = Vec::new();
  let mut overflow: u16 = 0;
  for digit in digits {
    let n: u16 = (digit as u16) * f + overflow;
    result.push((n % 10) as u8);
    overflow = n / 10;
  }
  while overflow > 0 {
    result.push((overflow % 10) as u8);
    overflow /= 10;
  }
  result
}
