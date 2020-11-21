// Power digit sum
//
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 2^1000?

#[test]
fn test() {
  assert_eq!(power2_digit_sum(15), 26);
}

pub fn solve() -> u16 {
  power2_digit_sum(1_000)
}

fn power2_digit_sum(power: u16) -> u16 {
  let mut digits: Vec<u8> = vec![1; 1];
  let mut power_remaining: u16 = power;
  while power_remaining > 0 {
    double(&mut digits);
    power_remaining -= 1
  }
  digits.iter().map(|&d| d as u16).sum()
}

fn double(digits_reversed: &mut Vec<u8>) {
  let mut overflow: u8 = 0;
  for index in 0..digits_reversed.len() {
    let digit = digits_reversed[index] * 2 + overflow;
    overflow = digit / 10;
    digits_reversed[index] = digit % 10;
  }
  if overflow > 0 {
    digits_reversed.push(overflow);
  }
}
