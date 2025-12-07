extern crate alloc;

use alloc::vec::Vec;
use day_6::prelude::Operand;


pub struct Problem {
  #[cfg(not(feature = "part2"))]
  pub rows: Vec<Vec<u64>>,
  #[cfg(feature = "part2")]
  pub rows: Vec<String>,
  pub operands: Vec<Operand>,
}

#[cfg(feature = "part2")]
impl Problem {
  #[allow(nonstandard_style)]
  pub fn get_parameters(&self) -> (usize, usize, usize) {
    let W = self.rows[0].len();
    let G = self.operands.len();
    let R = self.rows.len();

    (W, G, R)
  }
}

#[cfg(not(feature = "part2"))]
pub fn parse(input: &str) -> Problem {
  let mut lines: Vec<&str> =
    input.lines().filter(|l| !l.trim().is_empty()).collect();

  if lines.is_empty() {
    return Problem { rows: Vec::new(), operands: Vec::new() };
  }

  let op_line = lines.pop().expect("Input must have operands line");

  let operands: Vec<Operand> = op_line
    .split_whitespace()
    .map(|op_str| match op_str {
      "*" => Operand::Multiplication,
      "/" => Operand::Division,
      "+" => Operand::Addition,
      "-" => Operand::Subtraction,
      _ => panic!("unknown operand"),
    })
    .collect();

  let rows: Vec<Vec<u64>> = lines
    .iter()
    .map(|line| {
      line
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
    })
    .collect();

  Problem { rows, operands }
}

#[cfg(feature = "part2")]
pub fn parse(input: &str) -> Problem {
  let mut lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

  if lines.is_empty() {
    return Problem { rows: Vec::new(), operands: Vec::new() };
  }

  let op_line = lines.pop().expect("No operands line");

  let operands: Vec<Operand> = op_line
    .split_whitespace()
    .map(|op_str| match op_str {
      "*" => Operand::Multiplication,
      "/" => Operand::Division,
      "+" => Operand::Addition,
      "-" => Operand::Subtraction,
      _ => panic!("unknown operand"),
    })
    .collect();

  let rows: Vec<String> = lines.into_iter().map(String::from).collect();

  Problem { rows, operands }
}
