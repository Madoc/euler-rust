// Special Pythagorean triplet

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

#[test]
fn test() {
  assert_eq!(pythagorean_triplet_product(3 + 4 + 5), 3 * 4 * 5)
}

// TODO too slow
pub fn solve() -> u64 {
  pythagorean_triplet_product(1000)
}

fn pythagorean_triplet_product(triplet_sum: u64) -> u64 {
  for a in 1..triplet_sum {
    for b in a + 1..triplet_sum {
      for c in b + 1..triplet_sum {
        if is_pythagorean_triplet(a, b, c) && a + b + c == triplet_sum {
          return a * b * c;
        }
      }
    }
  }
  0
}

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
  a * a + b * b == c * c
}
