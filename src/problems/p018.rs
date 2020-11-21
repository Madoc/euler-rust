// Maximum path sum I
//
// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from
// top to bottom is 23.
//
// 3
// 7 4
// 2 4 6
// 8 5 9 3
//
// That is, 3 + 7 + 4 + 9 = 23.
//
// Find the maximum total from top to bottom of the triangle below:
//
// 75
// 95 64
// 17 47 82
// 18 35 87 10
// 20 04 82 47 65
// 19 01 23 75 03 34
// 88 02 77 73 07 63 67
// 99 65 04 28 06 16 70 92
// 41 41 26 56 83 40 80 70 33
// 41 48 72 33 47 32 37 16 94 29
// 53 71 44 65 25 43 91 52 97 51 14
// 70 11 33 28 77 73 17 78 39 68 17 57
// 91 71 52 38 17 14 91 43 58 50 27 29 48
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
//
// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem
// 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and
// requires a clever method! ;o)

type CellValue = u8;
type PathSum = u16;

#[allow(dead_code)]
const TEST_PYRAMID: [[CellValue; 4]; 4] = [[3, 0, 0, 0], [7, 4, 0, 0], [2, 4, 6, 0], [8, 5, 9, 3]];

const PROBLEM_PYRAMID: [[CellValue; 15]; 15] = [
  [75, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [95, 64, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [17, 47, 82, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [18, 35, 87, 10, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [20, 04, 82, 47, 65, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [19, 01, 23, 75, 03, 34, 00, 00, 00, 00, 00, 00, 00, 00, 00],
  [88, 02, 77, 73, 07, 63, 67, 00, 00, 00, 00, 00, 00, 00, 00],
  [99, 65, 04, 28, 06, 16, 70, 92, 00, 00, 00, 00, 00, 00, 00],
  [41, 41, 26, 56, 83, 40, 80, 70, 33, 00, 00, 00, 00, 00, 00],
  [41, 48, 72, 33, 47, 32, 37, 16, 94, 29, 00, 00, 00, 00, 00],
  [53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 00, 00, 00, 00],
  [70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, 00, 00, 00],
  [91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 00, 00],
  [63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 00],
  [04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
];

#[test]
fn test() {
  assert_eq!(
    pyramid_path_max(&TEST_PYRAMID.iter().map(|&l| l.to_vec()).collect()),
    23
  );
}

pub fn solve() -> PathSum {
  pyramid_path_max(&PROBLEM_PYRAMID.iter().map(|&l| l.to_vec()).collect())
}

#[derive(Clone, Copy, Debug)]
struct Candidate {
  x: usize,
  path_sum: PathSum,
}

fn pyramid_path_max(pyramid: &Vec<Vec<CellValue>>) -> PathSum {
  run_pyramid(&pyramid.iter().map(|l| l.to_vec()).collect())
    .iter()
    .map(|c| c.path_sum)
    .max()
    .unwrap()
}

fn run_pyramid(pyramid: &Vec<Vec<CellValue>>) -> Vec<Candidate> {
  let mut candidates: Vec<Candidate> = vec![Candidate {
    x: 0,
    path_sum: pyramid[0][0] as PathSum,
  }];
  let cell_max: CellValue = *pyramid.iter().map(|l| l.iter().max().unwrap()).max().unwrap();
  let cell_min: CellValue = *pyramid
    .iter()
    .map(|l| l.iter().filter(|c| **c != 0).min().unwrap())
    .min()
    .unwrap();

  for y in 1..pyramid.len() {
    candidates = extend(&candidates, &pyramid[y]);
    let remaining_y = pyramid.len() - y - 1;
    candidates = remove_hopeless(
      &candidates,
      cell_min as PathSum * remaining_y as PathSum,
      cell_max as PathSum * remaining_y as PathSum,
    );
  }

  candidates
}

fn extend(candidates: &Vec<Candidate>, cells: &Vec<CellValue>) -> Vec<Candidate> {
  let mut extended: Vec<Candidate> = Vec::new();
  // Create two follow-ups for each candidate:
  for candidate in candidates {
    extended.push(Candidate {
      x: candidate.x,
      path_sum: candidate.path_sum + cells[candidate.x as usize] as PathSum,
    });
    if candidate.x < cells.len() {
      extended.push(Candidate {
        x: candidate.x + 1,
        path_sum: candidate.path_sum + cells[(candidate.x + 1) as usize] as PathSum,
      });
    }
  }
  // For every two candidates with the same x, remove the one with the lower path sum:
  for index in (extended.len() - 1..0).rev() {
    if extended[index].x == extended[index + 1].x {
      if extended[index].path_sum < extended[index + 1].path_sum {
        extended.remove(index);
      } else {
        extended.remove(index + 1);
      }
    }
  }
  extended
}

fn remove_hopeless(candidates: &Vec<Candidate>, remaining_min: PathSum, remaining_max: PathSum) -> Vec<Candidate> {
  let best_min: PathSum = candidates.iter().map(|c| c.path_sum).max().unwrap() + remaining_min;
  candidates
    .iter()
    .filter(|c| c.path_sum + remaining_max >= best_min)
    .map(|c| *c)
    .collect()
}
