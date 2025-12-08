#![cfg_attr(not(feature = "std"), no_std)]

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
// Configuration
// --------------------------

const MAX_POINTS: usize = 1000;
#[cfg(not(feature = "part2"))]
const MAX_EDGES: usize = 1000;

#[cfg(all(feature = "sample", not(feature = "part2")))]
pub const K_EDGES: usize = 10;

#[cfg(all(not(feature = "sample"), not(feature = "part2")))]
pub const K_EDGES: usize = 1000;

// --------------------------
// Data Model
// --------------------------

pub struct Problem<'a> {
  pub input: &'a str,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ThreeSpacePoint {
  x: i32,
  y: i32,
  z: i32,
}

impl From<&str> for ThreeSpacePoint {
  fn from(value: &str) -> Self {
    let mut positions = value.trim().split(',');
    let x = positions.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let y = positions.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let z = positions.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    ThreeSpacePoint { x, y, z }
  }
}

// Stores (squared_distance, index_from, index_to)
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HeapItem((u64, usize, usize));

impl Ord for HeapItem {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.0.0.cmp(&other.0.0)
  }
}

impl PartialOrd for HeapItem {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl HeapItem {
  pub fn from_points(
    from: &ThreeSpacePoint,
    to: &ThreeSpacePoint,
    i: usize,
    j: usize,
  ) -> HeapItem {
    let dx = (from.x - to.x) as i64;
    let dy = (from.y - to.y) as i64;
    let dz = (from.z - to.z) as i64;
    let distance = (dx * dx + dy * dy + dz * dz) as u64;
    HeapItem((distance, i, j))
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
  use heapless::binary_heap::{BinaryHeap, Max};

  use super::{
    HeapItem, K_EDGES, MAX_EDGES, MAX_POINTS, Problem, ThreeSpacePoint,
  };


  struct Dsu<'a> {
    parent: &'a mut [usize],
    size: &'a mut [u16],
  }

  impl<'a> Dsu<'a> {
    fn new(parent: &'a mut [usize], size: &'a mut [u16], n: usize) -> Self {
      for i in 0..n {
        parent[i] = i;
        size[i] = 1;
      }
      Dsu { parent, size }
    }

    // Find with Path Compression
    fn find(&mut self, i: usize) -> usize {
      let mut root = i;
      while root != self.parent[root] {
        root = self.parent[root];
      }
      let mut curr = i;
      while curr != root {
        let next = self.parent[curr];
        self.parent[curr] = root;
        curr = next;
      }
      root
    }

    // Union by Size
    fn union(&mut self, i: usize, j: usize) {
      let root_i = self.find(i);
      let root_j = self.find(j);

      if root_i != root_j {
        if self.size[root_i] < self.size[root_j] {
          self.parent[root_i] = root_j;
          self.size[root_j] += self.size[root_i];
        } else {
          self.parent[root_j] = root_i;
          self.size[root_i] += self.size[root_j];
        }
      }
    }
  }


  pub fn part1(p: &Problem) -> u64 {
    let mut points: Vec<ThreeSpacePoint, MAX_POINTS> = Vec::new();
    for line in p.input.lines() {
      let _ = points.push(ThreeSpacePoint::from(line));
    }

    let mut max_heap: BinaryHeap<HeapItem, Max, MAX_EDGES> = BinaryHeap::new();

    for i in 0..points.len() {
      for j in i + 1..points.len() {
        let item = HeapItem::from_points(&points[i], &points[j], i, j);

        if max_heap.len() < K_EDGES {
          let _ = max_heap.push(item);
        } else if let Some(top) = max_heap.peek() {
          if item.0.0 < top.0.0 {
            let _ = max_heap.pop();
            let _ = max_heap.push(item);
          }
        }
      }
    }

    let mut parent = [0usize; MAX_POINTS];
    let mut size = [0u16; MAX_POINTS];
    let mut dsu = Dsu::new(&mut parent, &mut size, points.len());

    while let Some(HeapItem((_, u, v))) = max_heap.pop() {
      dsu.union(u, v);
    }

    let mut size_heap: BinaryHeap<u16, Max, MAX_POINTS> = BinaryHeap::new();
    for i in 0..points.len() {
      // Only push the size of the root to avoid duplicates
      if dsu.parent[i] == i {
        let _ = size_heap.push(dsu.size[i]);
      }
    }

    let first = (size_heap.pop().unwrap_or(0)) as u64;
    let second = (size_heap.pop().unwrap_or(0)) as u64;
    let third = (size_heap.pop().unwrap_or(0)) as u64;

    first * second * third
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use heapless::Vec;

  use super::{MAX_POINTS, Problem, ThreeSpacePoint};

  // Prim's algorithm to avoid allocating the full edge table on the stack.
  // tracking each vertex the best connecting edge and its distance.
  pub fn part2(p: &Problem) -> u64 {
    let mut points: Vec<ThreeSpacePoint, MAX_POINTS> = Vec::new();
    for line in p.input.lines() {
      let _ = points.push(ThreeSpacePoint::from(line));
    }

    let n = points.len();
    if n < 2 {
      panic!("Not enough points for part2");
    }

    // Arrays sized to MAX_POINTS to avoid dynamic allocation; we only use indices < n.
    let mut min_dist = [u64::MAX; MAX_POINTS];
    let mut parent = [usize::MAX; MAX_POINTS];
    let mut visited = [false; MAX_POINTS];

    // Start from node 0
    min_dist[0] = 0;
    parent[0] = 0;

    let mut last_u = 0usize;
    let mut last_v = 0usize;

    for _ in 0..n {
      let mut u = None;
      let mut best = u64::MAX;
      for i in 0..n {
        if !visited[i] && min_dist[i] < best {
          best = min_dist[i];
          u = Some(i);
        }
      }

      let u = match u {
        Some(idx) => idx,
        None => break,
      };

      visited[u] = true;

      if parent[u] != u {
        last_u = parent[u];
        last_v = u;
      }

      for v in 0..n {
        if visited[v] || v == u {
          continue;
        }

        let dx = (points[u].x - points[v].x) as i64;
        let dy = (points[u].y - points[v].y) as i64;
        let dz = (points[u].z - points[v].z) as i64;
        let dist = (dx * dx + dy * dy + dz * dz) as u64;

        if dist < min_dist[v] {
          min_dist[v] = dist;
          parent[v] = u;
        }
      }
    }

    (points[last_u].x as u64) * (points[last_v].x as u64)
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

    assert_eq!(result, 40);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part2(&problem);

    assert_eq!(result, 25272);
  }
}
