#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod range;
pub mod u64_handlers;

use range::RangeIter;
use u64_handlers::write_u64_into_buf;


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
pub mod part1_impl {
  use super::{Problem, RangeIter, write_u64_into_buf};

  fn is_repeated_pattern(s: &str) -> bool {
    let len = s.len();

    if len % 2 != 0 {
      return false;
    }

    if s.as_bytes()[0] == b'0' {
      return false;
    }

    let half = len / 2;
    let (left, right) = s.split_at(half);

    left == right
  }

  pub fn part1(p: &Problem) -> u64 {
    let mut total: u64 = 0;
    let mut buf = [0u8; 20];

    for (from, to) in RangeIter::new(p.input.trim()) {
      for n in from..=to {
        let s = write_u64_into_buf(n, &mut buf);
        if is_repeated_pattern(s) {
          total += n as u64;
        }
      }
    }

    total
  }
}

#[cfg(not(feature = "part2"))]
pub use part1_impl::part1;

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
pub mod part2_impl {
  use super::{Problem, RangeIter, write_u64_into_buf};

  fn is_repeated_at_least_twice(s: &str) -> bool {
    let n = s.len();

    if n < 2 {
      return false;
    }

    let bytes = s.as_bytes();

    for d in 1..=n / 2 {
      if !n.is_multiple_of(d) {
        continue;
      }

      let chunk = &bytes[..d];
      let mut is_valid = true;

      let repeats = n / d;

      if repeats < 2 {
        continue;
      }

      for i in 1..repeats {
        let start = i * d;
        let end = start + d;
        if &bytes[start..end] != chunk {
          is_valid = false;
          break;
        }
      }

      if is_valid {
        return true;
      }
    }

    false
  }

  pub fn part2(p: &Problem) -> u64 {
    let mut total: u64 = 0;
    let mut buf = [0u8; 20];

    for (from, to) in RangeIter::new(p.input.trim()) {
      for n in from..=to {
        let s = write_u64_into_buf(n, &mut buf);
        if is_repeated_at_least_twice(s) {
          total += n;
        }
      }
    }

    total
  }
}

#[cfg(feature = "part2")]
pub use part2_impl::part2;


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let sample = include_str!("../sample.txt");
    let problem = parse(sample);
    let result = part1(&problem);
    assert_eq!(result, 1227775554);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let sample = include_str!("../sample.txt");
    let problem = parse(sample);
    let result = part2(&problem);
    assert_eq!(result, 4174379265);
  }
}
