#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use core::fmt::Debug;

#[cfg(feature = "std")]
extern crate std;


pub mod prelude {
  pub use crate::{Problem, parse};

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}


// --------------------------
// Data Model
// --------------------------

#[cfg(feature = "sample")]
const N: usize = 20;
#[cfg(not(feature = "sample"))]
const N: usize = 512;


pub struct Problem<'a> {
  pub input: &'a str,
}

#[derive(Copy, Clone)]
pub struct Coordinate {
  x: i32,
  y: i32,
}

impl Coordinate {
  #[inline]
  pub fn to_tuple(&self) -> (i32, i32) {
    (self.x, self.y)
  }
}

impl From<&str> for Coordinate {
  fn from(value: &str) -> Self {
    let mut positions = value.trim().split(',');
    let x = positions.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let y = positions.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    Coordinate { x, y }
  }
}

#[cfg(feature = "std")]
impl Debug for Coordinate {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "Coordinate(x {}, y {})", self.x, self.y)
  }
}

pub struct Pairwise {
  i: usize,
  j: usize,
  n: usize,
}

impl Pairwise {
  pub const fn new(n: usize) -> Self {
    Self { i: 0, j: 1, n }
  }
}

impl Iterator for Pairwise {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    if self.i + 1 >= self.n {
      return None;
    }

    let out = (self.i, self.j);

    if self.j + 1 < self.n {
      self.j += 1;
    } else {
      self.i += 1;
      self.j = self.i + 1;
    }

    Some(out)
  }
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
  use heapless::Vec;

  use super::{Coordinate, N, Pairwise, Problem};


  pub fn part1(p: &Problem) -> u64 {
    let coords: Vec<Coordinate, N> = p
      .input
      .lines()
      .map(|line| Coordinate::from(line.trim()))
      .collect();

    #[cfg(feature = "std")]
    dbg!(&coords);

    let mut acc: u64 = 0;

    for (i, j) in Pairwise::new(coords.len()) {
      let (x1, y1) = coords[i].to_tuple();
      let (x2, y2) = coords[j].to_tuple();

      let dx = (x1 - x2).abs() as u64 + 1;
      let dy = (y1 - y2).abs() as u64 + 1;
      let area = dx * dy;

      if area > acc {
        acc = area;
      }
    }

    acc
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use heapless::Vec;

  use super::{Coordinate, N, Pairwise, Problem};


  // (A, B_min, B_max)
  type VEdge = (i32, i32, i32);
  type HEdge = (i32, i32, i32);

  // Ray-casting - returns true if midpoint is inside polygon or on boundary.
  fn point_in_poly_doubled(
    mx2: i64,
    my2: i64,
    v_edges: &[VEdge],
    h_edges: &[HEdge],
  ) -> bool {
    // Boundary check first
    for &(vx, vy0, vy1) in v_edges {
      let vx2 = 2 * (vx as i64);
      let vy0_2 = 2 * (vy0 as i64);
      let vy1_2 = 2 * (vy1 as i64);
      if vx2 == mx2 && my2 >= vy0_2 && my2 <= vy1_2 {
        return true;
      }
    }

    for &(hy, hx0, hx1) in h_edges {
      let hy2 = 2 * (hy as i64);
      let hx0_2 = 2 * (hx0 as i64);
      let hx1_2 = 2 * (hx1 as i64);
      if hy2 == my2 && mx2 >= hx0_2 && mx2 <= hx1_2 {
        return true;
      }
    }

    let mut crossings = 0;
    for &(vx, vy0, vy1) in v_edges {
      let vx2 = 2 * (vx as i64);
      if vx2 <= mx2 {
        continue;
      }

      let vy0_2 = 2 * (vy0 as i64);
      let vy1_2 = 2 * (vy1 as i64);

      // include-lower / exclude-upper bound rule
      if my2 >= vy0_2 && my2 < vy1_2 {
        crossings += 1;
      }
    }

    crossings % 2 == 1
  }

  // Rectangle–edge interior intersection check
  // false if polygon edges enter rectangle interior
  fn rect_is_clear_of_edges(
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    v_edges: &[VEdge],
    h_edges: &[HEdge],
  ) -> bool {
    // Check vertical edges that lie strictly between x1,x2
    for &(vx, vy0, vy1) in v_edges {
      if vx > x1 && vx < x2 && vy0.max(y1) < vy1.min(y2) {
        return false;
      }
    }

    // Check horizontal edges that lie strictly between y1,y2
    for &(hy, hx0, hx1) in h_edges {
      if hy > y1 && hy < y2 && hx0.max(x1) < hx1.min(x2) {
        return false;
      }
    }

    true
  }

  pub fn part2(p: &Problem) -> u64 {
    let coords: Vec<Coordinate, N> = p
      .input
      .lines()
      .map(|line| Coordinate::from(line.trim()))
      .collect();

    let n = coords.len();

    let mut v_edges: Vec<VEdge, N> = Vec::new();
    let mut h_edges: Vec<HEdge, N> = Vec::new();

    for i in 0..n {
      let c1 = coords[i];
      let c2 = coords[(i + 1) % n];
      if c1.x == c2.x {
        let _ = v_edges.push((c1.x, c1.y.min(c2.y), c1.y.max(c2.y)));
      } else {
        let _ = h_edges.push((c1.y, c1.x.min(c2.x), c1.x.max(c2.x)));
      }
    }

    let mut max_area: u64 = 0;

    for (i, j) in Pairwise::new(n) {
      let c1 = coords[i];
      let c2 = coords[j];

      let xa = c1.x.min(c2.x);
      let xb = c1.x.max(c2.x);
      let ya = c1.y.min(c2.y);
      let yb = c1.y.max(c2.y);

      // Crossing test
      if !rect_is_clear_of_edges(xa, xb, ya, yb, &v_edges, &h_edges) {
        continue;
      }

      // Midpoint test
      let mx2 = xa as i64 + xb as i64;
      let my2 = ya as i64 + yb as i64;

      if !point_in_poly_doubled(mx2, my2, &v_edges, &h_edges) {
        continue;
      }

      let dx = (xb - xa + 1) as u64;
      let dy = (yb - ya + 1) as u64;
      let area = dx * dy;

      if area > max_area {
        max_area = area;
      }
    }

    max_area
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

    assert_eq!(result, 50);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part2(&problem);
    assert_eq!(result, 24);
  }
}
