// Names scores
//
// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first
// names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply
// this value by its alphabetical position in the list to obtain a name score.
//
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the
// 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
//
// What is the total of all the name scores in the file?

const NAMES: &'static str = include_str!("data/p022/p022_names.txt");

#[test]
fn test() {
  assert_eq!(parse_names()[937], "COLIN");
  assert_eq!(score("COLIN"), 53);
}

pub fn solve() -> u32 {
  let mut sum: u32 = 0;
  let mut count: u32 = 1;
  for name in parse_names() {
    sum += score(name) * count;
    count += 1;
  }
  sum
}

fn parse_names() -> Vec<&'static str> {
  let mut names: Vec<&str> = NAMES
    .split(",")
    .map(|n| n.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap())
    .collect();
  names.sort();
  names
}

fn score(name: &str) -> u32 {
  name.chars().map(|c| (c as u32) - ('A' as u32) + 1).sum()
}
