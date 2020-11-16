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

const FAST_CACHE_SIZE: usize = 4_000_000;

type CollatzNumber = u32;
type StepCount = u16;

#[test]
fn test() {
  assert_eq!(collatz_chain_length(13, &mut LinkedList::new(), &mut Cache::new()), 10)
}

pub fn solve() -> CollatzNumber {
  longest_start_n(1_000_000)
}

fn longest_start_n(up_to_excl: CollatzNumber) -> CollatzNumber {
  let mut cache: Cache = Cache::new();
  (1..up_to_excl)
    .map(|n| (n, collatz_chain_length(n, &mut LinkedList::new(), &mut cache)))
    .max_by_key(|t| t.1)
    .map(|t| t.0)
    .unwrap()
}

fn collatz_chain_length(
  n: CollatzNumber,
  previous_seq: &mut LinkedList<CollatzNumber>,
  cache: &mut Cache,
) -> StepCount {
  if n == 1 || cache.contains_key(n) {
    let mut steps: StepCount = cache.get(n) + (previous_seq.len() as StepCount);
    let result = steps;
    while !previous_seq.is_empty() {
      steps -= 1;
      cache.insert(previous_seq.pop_front().unwrap(), steps);
    }
    result
  } else {
    let next_n: CollatzNumber = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    previous_seq.push_back(n);
    collatz_chain_length(next_n, previous_seq, cache)
  }
}

struct Cache {
  fast: Box<[StepCount]>,
  slow: HashMap<CollatzNumber, StepCount>,
}
impl Cache {
  fn new() -> Cache {
    let mut result: Cache = Cache {
      fast: vec![0 as StepCount; FAST_CACHE_SIZE].into_boxed_slice(),
      slow: HashMap::new(),
    };
    result.insert(1, 1);
    result
  }

  fn contains_key(&self, key: CollatzNumber) -> bool {
    if (key as usize) < FAST_CACHE_SIZE {
      self.fast[(key - 1) as usize] != 0
    } else {
      self.slow.contains_key(&key)
    }
  }

  fn get(&self, key: CollatzNumber) -> StepCount {
    if (key as usize) < FAST_CACHE_SIZE {
      self.fast[(key - 1) as usize]
    } else {
      self.slow[&key]
    }
  }

  fn insert(&mut self, key: CollatzNumber, value: StepCount) {
    if (key as usize) < FAST_CACHE_SIZE {
      self.fast[(key - 1) as usize] = value;
    } else {
      self.slow.insert(key, value);
    }
  }
}
