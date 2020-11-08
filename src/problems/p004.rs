// Largest palindrome product
//
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is
// 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

#[test]
fn test() {
  assert_eq!(largest_palindromic_product(2), 9009)
}

pub fn solve() -> u32 {
  largest_palindromic_product(3)
}

fn largest_palindromic_product(factor_digits: u8) -> u32 {
  let min_factor: u32 = 10u32.pow((factor_digits - 1) as u32);
  let max_factor: u32 = 10u32.pow(factor_digits as u32) - 1;
  let mut largest_palindromic_product: u32 = 0;
  for n1 in min_factor..=max_factor {
    for n2 in n1..=max_factor {
      let product = n1 * n2;
      if product > largest_palindromic_product && is_palindromic(product) {
        largest_palindromic_product = product
      }
    }
  }
  largest_palindromic_product
}

fn is_palindromic(n: u32) -> bool {
  let str: String = n.to_string();
  str == str.chars().rev().collect::<String>()
}
