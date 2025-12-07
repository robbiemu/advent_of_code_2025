#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
  pub use crate::{parse, Problem};

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}

// --------------------------
// Data Model
// --------------------------

pub struct Problem<'a> {
  pub input: &'a str,
}

// --------------------------
// Parse
// --------------------------

pub fn parse(input: &str) -> Problem<'_> {
  Problem { input }
}

// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::Problem;

  pub fn part1(p: &Problem) -> u64 {
    // Replace with your Day 1 logic
    0
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use super::Problem;

  pub fn part2(p: &Problem) -> u64 {
    // Replace with your Day 1 Part 2 logic
    0
  }
}

#[cfg(test)]
mod tests {
  use super::prelude::*;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part1(&problem);

    assert_eq!(result, todo!("result assertion needed"));
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part2(&problem);
    assert_eq!(result, todo!("result assertion needed"));
  }
}
