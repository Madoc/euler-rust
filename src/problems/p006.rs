// Sum square difference
//
// The sum of the squares of the first ten natural numbers is,
//
// 1^2 + 2^2 + ... + 10^2 = 385
//
// The square of the sum of the first ten natural numbers is,
//
// (1 + 2 + ... 10)^2 = 55^2 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
// 3025 - 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the
// sum.

#[test]
fn test() {
  assert_eq!(diff_of_sums(10), 2640)
}

pub fn solve() -> u64 {
  diff_of_sums(100)
}

fn diff_of_sums(max: u64) -> u64 {
  let mut sum_of_squares = 0;
  let mut square_of_sum = 0;
  for i in 1..=max {
    sum_of_squares += i * i;
    square_of_sum += i;
  }
  square_of_sum *= square_of_sum;
  square_of_sum - sum_of_squares
}
