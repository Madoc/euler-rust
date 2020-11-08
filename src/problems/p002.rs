// Even Fibonacci numbers
//
// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the
// first 10 terms will be:
//
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the
// even-valued terms.

#[test]
fn test() {
  assert_eq!(even_fibonacci_sum_up_to(89), 44)
}

pub fn solve() -> u32 {
  even_fibonacci_sum_up_to(4_000_000)
}

fn even_fibonacci_sum_up_to(limit: u32) -> u32 {
  let mut result: u32 = 0;
  let mut t1: u32 = 1;
  let mut t2: u32 = 2;
  while t1 <= limit {
    if t1 % 2 == 0 {
      result += t1
    }
    let sum: u32 = t1 + t2;
    t1 = t2;
    t2 = sum;
  }
  result
}
