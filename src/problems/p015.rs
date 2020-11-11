// Lattice paths
//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6
// routes to the bottom right corner.
//
// How many such routes are there through a 20×20 grid?

// Implementation note: This problem can be solved more easily with mathematics, by turning it by 45°. However, this
// solution models the problem as described, for the fun of it.

type Count = u64;

#[test]
fn test() {
  assert_eq!(num_routes(3, 3), 6);
}

pub fn solve() -> Count {
  num_routes(21, 21) // There are 20 boxes, but 21 points.
}

struct Field {
  height: usize,
  width: usize,
  counts_is_a: bool, // Double buffering between counts_a and counts_b.
  counts_a: Vec<Count>,
  counts_b: Vec<Count>,
}
impl Field {
  fn new(width: usize, height: usize) -> Field {
    Field {
      height,
      width,
      counts_a: vec![0; width * height],
      counts_b: vec![0; width * height],
      counts_is_a: true,
    }
  }

  fn counts(&self) -> &Vec<Count> {
    if self.counts_is_a {
      &self.counts_a
    } else {
      &self.counts_b
    }
  }

  fn counts_swap_mut(&mut self) -> &mut Vec<Count> {
    if self.counts_is_a {
      &mut self.counts_b
    } else {
      &mut self.counts_a
    }
  }

  fn add(&mut self, x: usize, y: usize, count: Count) {
    let index = y * self.width + x;
    self.counts_swap_mut()[index] += count;
  }

  fn get(&self, x: usize, y: usize) -> Count {
    self.counts()[y * self.width + x]
  }

  fn swap(&mut self) {
    self.counts_is_a = !self.counts_is_a;
    let new_swap = self.counts_swap_mut();
    for i in 0..new_swap.len() {
      new_swap[i] = 0;
    }
  }
}

fn num_routes(width: usize, height: usize) -> Count {
  let mut field: Field = Field::new(width, height);
  field.add(0, 0, 1);
  field.swap();
  loop {
    one_move(&mut field);
    match check_finished(&field) {
      None => (),
      Some(result) => return result,
    }
  }
}

fn one_move(field: &mut Field) {
  let (height, width) = (field.height, field.width);
  for x in 0..width {
    for y in 0..height {
      let count = field.get(x, y);
      if count > 0 {
        if x < (width - 1) {
          field.add(x + 1, y, count);
        }
        if y < (height - 1) {
          field.add(x, y + 1, count);
        }
      }
    }
  }
  field.swap();
}

fn check_finished(field: &Field) -> Option<Count> {
  let (height, width) = (field.height, field.width);
  for x in 0..width {
    for y in 0..height {
      if (x != width - 1 || y != height - 1) && field.get(x, y) != 0 {
        return None;
      }
    }
  }
  Some(field.get(width - 1, height - 1))
}
