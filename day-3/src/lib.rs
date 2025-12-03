#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

// using precomputed powers of 10 for efficiency in embedded/no_std contexts
#[cfg(feature = "part2")]
pub const POW10: [u64; 12] = [
  1,
  10,
  100,
  1_000,
  10_000,
  100_000,
  1_000_000,
  10_000_000,
  100_000_000,
  1_000_000_000,
  10_000_000_000,
  100_000_000_000,
];

pub mod prelude {
  #[cfg(feature = "part2")]
  pub use crate::POW10;
  pub use crate::{Problem, parse};

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

  pub fn find_max_joltage(bank: &str) -> usize {
    let bytes = bank.as_bytes();

    let mut first = true;
    let mut best_right = 0usize;
    let mut acc = 0usize;

    for &b in bytes.iter().rev() {
      let tens_digit = (b - b'0') as usize;

      let already_seen_digit = !first;
      let not_leading_zero = tens_digit != 0;
      if already_seen_digit && not_leading_zero {
        let candidate = tens_digit * 10 + best_right;
        if candidate > acc {
          acc = candidate;
        }
      }
      first = false;

      if tens_digit > best_right {
        best_right = tens_digit;
      }
    }

    acc
  }


  pub fn part1(p: &Problem) -> u64 {
    p.input
      .lines()
      .map(|line| {
        let j = find_max_joltage(line) as u64;

        #[cfg(feature = "std")]
        eprintln!("Line: '{}', max joltage: {}", line, j);

        j
      })
      .sum()
  }
}

// --------------------------
// Solver — Part 2
// --------------------------
#[cfg(feature = "part2")]
mod part2_impl {
  use super::{POW10, Problem};

  pub fn find_max_joltage(bank: &str) -> u64 {
    let bytes = bank.as_bytes();

    if bytes.len() < 12 {
      return 0;
    }

    let mut best_right: [u64; 11] = [0; 11];
    let mut acc: u64 = 0;

    for (seen, &b) in bytes.iter().rev().enumerate() {
      let first_digit = (b - b'0') as u64;

      if best_right[10] != 0 {
        let candidate = first_digit * POW10[11] + best_right[10];
        if candidate > acc {
          acc = candidate;
        }
      }

      let mut new_best = best_right;

      // Update suffix length 1 using old best_right
      if first_digit > new_best[0] {
        new_best[0] = first_digit;
      }

      let max_k = core::cmp::min(seen, 10);

      for k in 1..=max_k {
        let new_suffix = first_digit * POW10[k] + best_right[k - 1];
        if new_suffix > new_best[k] {
          new_best[k] = new_suffix;
        }
      }

      best_right = new_best;
    }

    acc
  }

  pub fn part2(p: &Problem) -> u64 {
    p.input
      .lines()
      .map(|line| {
        let j = find_max_joltage(line);

        #[cfg(feature = "std")]
        eprintln!("Line: '{}', max joltage: {}", line, j);

        j
      })
      .sum()
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
    assert_eq!(result, 357);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part2(&problem);
    assert_eq!(result, 3121910778619);
  }
}
