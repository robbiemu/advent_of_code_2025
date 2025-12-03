#![cfg_attr(not(feature = "std"), no_std)]

// --------------------------
// Data Model
// --------------------------

pub enum Turn {
  Left,
  Right,
}

pub struct Instruction {
  dir: Turn,
  amount: u16,
}

pub struct Problem<'a> {
  pub input: &'a str,
}

// --------------------------
// Parse
// --------------------------

fn parse_instruction(line: &str) -> Instruction {
  let (dir, num_str) = line.split_at(1);

  let dir = match dir.as_bytes()[0] {
    b'L' => Turn::Left,
    b'R' => Turn::Right,
    _ => unimplemented!(
      "Turns must either be counter-clockwise (L) or clockwise (R)"
    ),
  };

  let amount = u16::from_str_radix(num_str, 10).unwrap();

  Instruction { dir, amount }
}


pub fn parse(input: &str) -> Problem<'_> {
  Problem { input }
}

// --------------------------
// Solver — Part 1
// --------------------------

#[cfg(not(feature = "part2"))]
pub mod part1_impl {
  use super::{parse_instruction, Problem, Turn};

  pub fn part1(p: &Problem) -> u64 {
    let mut pos: i16 = 50;
    let mut zeros = 0;

    for raw in p.input.lines() {
      #[cfg(feature = "std")]
      eprintln!("Processing line: '{}'\n", raw);
      let line = raw.trim();
      if line.is_empty() {
        continue;
      }

      let instruction = parse_instruction(line);

      pos = (pos
        + match instruction.dir {
          Turn::Left => -1 * (instruction.amount % 100) as i16,
          Turn::Right => (instruction.amount % 100) as i16,
        })
        % 100;

      if pos == 0 {
        zeros += 1;
      }
    }

    zeros
  }
}

#[cfg(not(feature = "part2"))]
pub use part1_impl::part1;

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
pub fn part2(p: &Problem) -> u64 {
  let mut pos: i16 = 50;
  let mut zeros = 0;

  for raw in p.input.lines() {
    #[cfg(feature = "std")]
    eprintln!("Processing line: '{}'\n", raw);
    let line = raw.trim();
    if line.is_empty() {
      continue;
    }

    let instruction = parse_instruction(line);

    let dir = &instruction.dir;
    let movement = instruction.amount as i16;

    let mut first = match dir {
      Turn::Right => 100 - pos,
      Turn::Left => pos,
    };

    // you only see 0 in passing after a full turn
    if first == 0 {
      first = 100;
    }

    let passes = if movement < first {
      0
    } else {
      1 + (movement - first) / 100
    };

    zeros += passes as u64;

    #[cfg(feature = "std")]
    let old_pos = pos;

    pos = (pos
      + match instruction.dir {
        Turn::Left => -((instruction.amount % 100) as i16),
        Turn::Right => (instruction.amount % 100) as i16,
      })
    .rem_euclid(100);

    #[cfg(feature = "std")]
    if passes > 0 {
      eprintln!(
        "Processing line: '{}' found {} passes ({} -> {})\n",
        raw, passes, old_pos, pos
      );
    }
  }

  zeros
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let sample = include_str!("../sample.txt");

    let p = parse(sample);
    let answer = part1(&p);

    assert_eq!(answer, 3);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let sample = include_str!("../sample.txt");
    let problem = parse(sample);
    let result = part2(&problem);
    assert_eq!(result, 6);
  }
}
