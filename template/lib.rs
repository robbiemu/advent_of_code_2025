#![cfg_attr(not(feature = "std")), no_std)]

#[cfg(feature = "std")]
extern crate std;

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

pub fn part1(p: &Problem) -> u64 {
  // Replace with your Day 1 logic
  0
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
pub fn part2(p: &Problem) -> u64 {
  // Replace with your Day 1 Part 2 logic
  0
}
