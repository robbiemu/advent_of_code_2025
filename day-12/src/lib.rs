#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

use heapless::Vec;

mod parser {
  include!("parser.rs");
}
use parser::{parse_regions, parse_shapes, skip_blank_lines};

mod data_model {
  include!("data_model.rs");
}
use data_model::{MAX_REGIONS, Region, SHAPES, Shape};


pub mod prelude {
  pub use crate::{Problem, parse};

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
  pub shapes: [Shape; SHAPES],
  pub regions: Vec<Region, MAX_REGIONS>,
}

#[cfg(feature = "part2")]
impl Problem {
  pub fn is_awesome(&self) -> bool {
    // Nothing to do, already awesome
    true
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
  Nom,
  BadShapeDims,
  RaggedShape,
  TooManyRegions,
  WrongShapeCount,
  WrongCountsCount,
  CountsOverflow,
}

pub fn parse(input: &str) -> Result<Problem, ParseError> {
  let (rest, shapes_vec) = parse_shapes(input).map_err(|_| ParseError::Nom)?;

  let shapes: [Shape; SHAPES] = shapes_vec
    .as_slice()
    .try_into()
    .map_err(|_| ParseError::WrongShapeCount)?;

  let (rest, _) = skip_blank_lines(rest).map_err(|_| ParseError::Nom)?;
  let (rest, regions) = parse_regions(rest).map_err(|_| ParseError::Nom)?;

  // allow trailing whitespace
  let _ = rest;

  Ok(Problem { shapes, regions })
}


// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::Problem;

  #[inline(always)]
  fn min_needed_space_for_count(
    #[cfg(feature = "std")] shape_idx: usize,
    #[cfg(feature = "std")] shape_id: u8,
    count: u16,
    pair_area: Option<usize>,
  ) -> usize {
    let n = count as usize;
    let single_area = 9usize; // 3x3 footprint per single

    match pair_area {
      None => n * single_area,
      Some(pa) => {
        let pairs = n / 2;
        let singles = n % 2;
        let best = pairs * pa + singles * single_area;

        #[cfg(feature = "std")]
        eprintln!(
          "shape_idx={} shape_id={} count={} pair_area={:?} => best={}",
          shape_idx, shape_id, count, pair_area, best
        );

        best
      }
    }
  }

  pub fn part1(p: &Problem) -> usize {
    p.regions
      .iter()
      .filter(|r| {
        let mut needed_space = 0usize;

        for (idx, &count) in r.counts.iter().enumerate() {
          if count == 0 {
            continue;
          }

          let shape = &p.shapes[idx];

          needed_space += min_needed_space_for_count(
            #[cfg(feature = "std")]
            idx,
            #[cfg(feature = "std")]
            shape.id,
            count,
            shape.tight_pair_area,
          );
        }

        let area = (r.w as usize) * (r.h as usize);

        #[cfg(feature = "std")]
        eprintln!(
          "region {}x{} needed_space={} area={}",
          r.w, r.h, needed_space, area
        );

        needed_space <= area
      })
      .count()
  }
}


// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use super::Problem;

  fn be_awesome(p: &Problem) -> usize {
    if p.is_awesome() { 42 } else { 0 }
  }

  pub fn part2(p: &Problem) -> usize {
    be_awesome(p)
  }
}

#[cfg(test)]
mod tests {
  use super::prelude::*;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let problem = match parse(input) {
      Ok(p) => p,
      Err(e) => panic!("Parse error: {:?}", e),
    };
    let result = part1(&problem);

    assert_eq!(result, 2);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let problem = match parse(input) {
      Ok(p) => p,
      Err(e) => panic!("Parse error: {:?}", e),
    };
    let result = part2(&problem);
    assert_eq!(result, 42);
  }
}
