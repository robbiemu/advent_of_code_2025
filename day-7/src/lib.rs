#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;


pub mod prelude {
  pub use crate::Problem;

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}

// --------------------------
// Data Model
// --------------------------

pub struct Problem<'a> {
  pub splitters: &'a [bool],
  pub start: (usize, usize),
  pub width: usize,
  pub height: usize,

  #[cfg(not(feature = "part2"))]
  pub buf: &'a mut [usize],
  #[cfg(not(feature = "part2"))]
  pub in_queue: &'a mut [bool],

  #[cfg(feature = "part2")]
  pub counts: &'a mut [u64],
}

// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use tinysetqueue::{MembershipMode, TinySetQueue};

  use super::Problem;

  pub fn part1(p: &mut Problem) -> u64 {
    let size = p.width * p.height;

    let mut queue =
      TinySetQueue::new(p.buf, p.in_queue, MembershipMode::Visited);

    let _ = queue.push(p.start.1 * p.width + p.start.0);

    let mut splits = 0;
    while let Some(idx) = queue.pop() {
      let x = idx % p.width;

      // step, then
      if idx + p.width >= size {
        continue;
      }
      let next_idx = idx + p.width;

      if p.splitters[next_idx] {
        splits += 1;

        if x > 0 {
          let _ = queue.push(next_idx - 1);
        }
        if x < p.width - 1 {
          let _ = queue.push(next_idx + 1);
        }
      } else {
        let _ = queue.push(next_idx);
      }
    }

    splits
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use super::Problem;

  pub fn part2(p: &mut Problem) -> u64 {
    if p.counts.len() < 2 * p.width {
      return 0;
    }

    p.counts.fill(0);

    let mut src_offset = 0;
    let mut dst_offset = p.width;

    p.counts[src_offset + p.start.0] = 1;

    for y in p.start.1..p.height - 1 {
      let row_start_idx = y * p.width;

      // reset the destination row
      let dst_slice = &mut p.counts[dst_offset..dst_offset + p.width];
      dst_slice.fill(0);

      for x in 0..p.width {
        let count = p.counts[src_offset + x];
        if count == 0 {
          continue;
        }

        if p.splitters[row_start_idx + x] {
          // split
          if x > 0 {
            p.counts[dst_offset + x - 1] += count;
          }
          if x < p.width - 1 {
            p.counts[dst_offset + x + 1] += count;
          }
        } else {
          // propograte down
          p.counts[dst_offset + x] += count;
        }
      }

      core::mem::swap(&mut src_offset, &mut dst_offset);
    }

    let final_row = &p.counts[src_offset..src_offset + p.width];

    final_row.iter().sum()
  }
}

#[cfg(test)]
mod tests {
  mod std_parse {
    include!("../src/std_parse.rs");
  }
  use super::prelude::*;
  use std_parse::parse;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let mut problem_data = parse(input);
    let mut problem = problem_data.as_problem();

    let result = part1(&mut problem);

    assert_eq!(result, 21);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let mut problem_data = parse(input);
    let mut problem = problem_data.as_problem();

    let result = part2(&mut problem);

    assert_eq!(result, 40);
  }
}
