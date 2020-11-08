// Multiples of 3 and 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these
// multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

#[test]
fn test() {
  assert_eq!(sum_of_3s_or_5s(10), 23)
}

pub fn solve() -> u32 {
  sum_of_3s_or_5s(1000)
}

fn sum_of_3s_or_5s(max: u32) -> u32 {
  let mut result: u32 = 0;
  for n in 1..max {
    if n % 3 == 0 || n % 5 == 0 {
      result += n
    }
  }
  result
}
