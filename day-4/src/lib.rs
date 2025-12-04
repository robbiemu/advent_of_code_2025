#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(feature = "part2")]
pub mod tinysetqueue;

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

#[cfg(not(feature = "part2"))]
pub struct Problem<'a> {
  pub input: &'a str,
}

#[cfg(feature = "part2")]
pub struct Problem<'a> {
  pub input: &'a str,
  pub width: usize,
  pub height: usize,
}


// --------------------------
// Parse
// --------------------------

#[cfg(not(feature = "part2"))]
pub fn parse(input: &str) -> Problem<'_> {
  Problem { input }
}

#[cfg(feature = "part2")]
pub fn parse<'a>(input: &'a str) -> Problem<'a> {
  let height = input.lines().count();
  let width = input.lines().next().unwrap_or("").len();

  Problem { input, width, height }
}


// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::Problem;

  fn process_row(prev: Option<&[u8]>, cur: &[u8], next: Option<&[u8]>) -> u64 {
    cur
      .iter()
      .enumerate()
      .map(|(i, &cell)| {
        let mut score = 0;
        if cell == b'@' {
          let adjacent = [
            // above row
            if i > 0 {
              prev.and_then(|p| p.get(i - 1))
            } else {
              None
            },
            prev.and_then(|p| p.get(i)),
            prev.and_then(|p| p.get(i + 1)),
            // same row
            if i > 0 { cur.get(i - 1) } else { None },
            cur.get(i + 1),
            // below row
            if i > 0 {
              next.and_then(|n| n.get(i - 1))
            } else {
              None
            },
            next.and_then(|n| n.get(i)),
            next.and_then(|n| n.get(i + 1)),
          ];

          let count_of_neighbors =
            adjacent.iter().filter(|&&c| c == Some(&b'@')).count();

          if count_of_neighbors < 4 {
            score += 1;
          }
        }
        score
      })
      .sum()
  }


  pub fn part1(p: &Problem) -> u64 {
    let mut prev: Option<&[u8]> = None;
    let mut cur: Option<&[u8]> = None;
    let mut total = 0;

    for next_line in p.input.lines().map(str::as_bytes) {
      if let Some(cur_line) = cur {
        total += process_row(prev, cur_line, Some(next_line));
      }

      prev = cur;
      cur = Some(next_line);
    }

    if let Some(cur_line) = cur {
      total += process_row(prev, cur_line, None);
    }

    total
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use super::Problem;
  use crate::tinysetqueue::TinySetQueue;

  #[allow(clippy::too_many_arguments)]
  fn compute_degree_row(
    y: usize,
    width: usize,
    prev: Option<&[u8]>,
    cur: &[u8],
    next: Option<&[u8]>,
    present: &mut [bool],
    degree: &mut [u8],
    queue: &mut TinySetQueue<usize>,
  ) {
    let idx_base = y * width;

    for x in 0..width {
      let idx = idx_base + x;

      if !present[idx] {
        degree[idx] = 0;
        continue;
      }

      let mut count = 0u8;

      // above row
      if let Some(p) = prev {
        if x > 0 && p[x - 1] == b'@' {
          count += 1;
        }
        if p[x] == b'@' {
          count += 1;
        }
        if x + 1 < width && p[x + 1] == b'@' {
          count += 1;
        }
      }

      // same row
      if x > 0 && cur[x - 1] == b'@' {
        count += 1;
      }
      if x + 1 < width && cur[x + 1] == b'@' {
        count += 1;
      }

      // below row
      if let Some(n) = next {
        if x > 0 && n[x - 1] == b'@' {
          count += 1;
        }
        if n[x] == b'@' {
          count += 1;
        }
        if x + 1 < width && n[x + 1] == b'@' {
          count += 1;
        }
      }

      degree[idx] = count;

      if count < 4 {
        queue
          .push(idx)
          .expect("queue overflow during initial degree compute");
      }
    }
  }

  #[inline]
  fn get_neighbors(
    idx: usize,
    width: usize,
    height: usize,
  ) -> impl Iterator<Item = usize> {
    let x = idx % width;
    let y = idx / width;

    let mut out = [usize::MAX; 8];
    let mut n = 0;

    // above row
    if y > 0 {
      let row = idx - width;
      if x > 0 {
        out[n] = row - 1;
        n += 1;
      }
      out[n] = row;
      n += 1;
      if x + 1 < width {
        out[n] = row + 1;
        n += 1;
      }
    }

    // same row
    if x > 0 {
      out[n] = idx - 1;
      n += 1;
    }
    if x + 1 < width {
      out[n] = idx + 1;
      n += 1;
    }

    // below row
    if y + 1 < height {
      let row = idx + width;
      if x > 0 {
        out[n] = row - 1;
        n += 1;
      }
      out[n] = row;
      n += 1;
      if x + 1 < width {
        out[n] = row + 1;
        n += 1;
      }
    }

    out.into_iter().take(n)
  }


  pub fn part2(
    p: &Problem,
    present: &mut [bool],
    degree: &mut [u8],
    queue: &mut TinySetQueue<usize>,
  ) -> u64 {
    let width = p.width;
    let height = p.height;

    let mut prev_line: Option<&[u8]> = None;
    let mut cur_line: Option<&[u8]> = None;
    let mut cur_y: Option<usize> = None;

    for (y, raw) in p.input.lines().enumerate() {
      let next_line = raw.as_bytes();

      // 1. Fill present[] for this row immediately
      let base = y * width;
      for x in 0..width {
        present[base + x] = next_line[x] == b'@';
      }

      // 2. When we have a current row, compute its degree using the available window
      if let (Some(cur), Some(cy)) = (cur_line, cur_y) {
        compute_degree_row(
          cy,
          width,
          prev_line,
          cur,
          Some(next_line),
          present,
          degree,
          queue,
        );

        // Slide the window forward after processing the middle row
        prev_line = cur_line;
      }

      cur_line = Some(next_line);
      cur_y = Some(y);
    }

    // 4. Flush the final row
    if let (Some(cur), Some(cy)) = (cur_line, cur_y) {
      compute_degree_row(
        cy,
        width,
        prev_line,
        cur,
        None,
        present,
        degree,
        queue,
      );
    }

    // progress along the frontier of adjacent removable cells
    let mut removed = 0;

    while let Some(idx) = queue.pop() {
      removed += 1;

      present[idx] = false;
      for n_idx in get_neighbors(idx, width, height) {
        if !present[n_idx] {
          continue;
        }

        degree[n_idx] -= 1;

        if degree[n_idx] < 4 {
          queue
            .push(n_idx)
            .expect("queue overflow during removal phase");
        }
      }
    }

    removed
  }
}

#[cfg(test)]
mod tests {
  use super::prelude::*;
  #[cfg(feature = "part2")]
  use std::vec;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part1(&problem);
    assert_eq!(result, 13);
  }

  // Part 2 works with externally allocated buffers, rather like a usb device controller or similar embedded problems
  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    use crate::tinysetqueue::prelude::{MembershipMode, TinySetQueue};

    let input = include_str!("../sample.txt");
    let problem = parse(input);

    let total = problem.width * problem.height;

    // working buffers
    let mut present = vec![false; total];
    let mut degree = vec![0u8; total];

    // queue storage and TinySetQueue
    let mut queue_buf = vec![0usize; total];
    let mut binding = vec![false; total];
    let mut queue =
      TinySetQueue::new(&mut queue_buf, &mut binding, MembershipMode::InQueue);

    let result = part2(&problem, &mut present, &mut degree, &mut queue);

    assert_eq!(result, 43);
  }
}
