extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::Problem;


pub struct ProblemData {
  splitters: Vec<bool>,
  start: (usize, usize),
  width: usize,
  height: usize,

  #[cfg(not(feature = "part2"))]
  buf: Vec<usize>,
  #[cfg(not(feature = "part2"))]
  in_queue: Vec<bool>,

  #[cfg(feature = "part2")]
  counts: Vec<u64>,
}

impl ProblemData {
  pub fn as_problem(&mut self) -> Problem<'_> {
    Problem {
      splitters: &self.splitters,
      start: self.start,
      width: self.width,
      height: self.height,

      #[cfg(not(feature = "part2"))]
      buf: &mut self.buf,
      #[cfg(not(feature = "part2"))]
      in_queue: &mut self.in_queue,

      #[cfg(feature = "part2")]
      counts: &mut self.counts,
    }
  }
}

pub fn parse(input: &str) -> ProblemData {
  let height = input.lines().count();
  let width = input.lines().next().map(|l| l.len()).unwrap_or(0);
  let size = width * height;

  let mut splitters = vec![false; size];
  let mut start = (0, 0);

  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      match ch {
        '^' => splitters[y * width + x] = true,
        'S' => start = (x, y),
        _ => {}
      }
    }
  }

  ProblemData {
    splitters,
    start,
    width,
    height,
    #[cfg(not(feature = "part2"))]
    buf: vec![0; size],
    #[cfg(not(feature = "part2"))]
    in_queue: vec![false; size],

    #[cfg(feature = "part2")]
    counts: vec![0; 2 * width],
  }
}
