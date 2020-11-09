// Longest Collatz sequence
//
// The following iterative sequence is defined for the set of positive integers:
//
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence:
//
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been
// proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

use std::collections::{HashMap, LinkedList};

type CollatzNumber = u64;
type StepCount = usize;

#[test]
fn test() {
  assert_eq!(
    collatz_chain_length(13, &mut LinkedList::new(), &mut HashMap::new()),
    10
  )
}

pub fn solve() -> CollatzNumber {
  longest_start_n(1_000_000)
}

fn longest_start_n(up_to_excl: CollatzNumber) -> CollatzNumber {
  let mut cache: HashMap<CollatzNumber, StepCount> = HashMap::new();
  (1..up_to_excl)
    .map(|n| (n, collatz_chain_length(n, &mut LinkedList::new(), &mut cache)))
    .max_by_key(|t| t.1)
    .map(|t| t.0)
    .expect("no results")
}

fn collatz_chain_length(
  n: CollatzNumber,
  previous_seq: &mut LinkedList<CollatzNumber>,
  cache: &mut HashMap<CollatzNumber, StepCount>,
) -> StepCount {
  if n == 1 || cache.contains_key(&n) {
    let mut steps: StepCount = *cache.get(&n).unwrap_or(&1) + previous_seq.len();
    let result = steps;
    while !previous_seq.is_empty() {
      steps -= 1;
      cache.insert(previous_seq.pop_front().expect("impossible"), steps);
    }
    result
  } else {
    let next_n: CollatzNumber = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    previous_seq.push_back(n);
    collatz_chain_length(next_n, previous_seq, cache)
  }
}
