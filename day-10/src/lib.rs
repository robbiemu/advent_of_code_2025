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
// Data Model
// --------------------------

pub struct Problem<'a> {
  pub input: &'a str,
}

const MAX_BUTTONS: usize = 15;
const MAX_LIGHTS: usize = 10;


pub fn parse(input: &str) -> Problem<'_> {
  Problem { input }
}


// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::{MAX_BUTTONS, MAX_LIGHTS, Problem};

  type Row = u16;


  struct Machine {
    num_lights: u8,
    num_buttons: u8,
    button_masks: [u16; MAX_BUTTONS],
    target_mask: u16,
  }


  fn parse_machine(line: &str) -> Machine {
    let mut parts = line.split_whitespace();
    let pattern = parts.next().expect("missing pattern");

    let inner = &pattern[1..pattern.len() - 1];
    let num_lights = inner.len() as u8;

    let target_mask = inner
      .bytes()
      .enumerate()
      .fold(0u16, |acc, (i, b)| acc | ((b == b'#') as u16) << i);

    let mut button_masks = [0u16; MAX_BUTTONS];
    let mut num_buttons = 0u8;

    for group in parts {
      if group.starts_with('{') {
        break;
      }

      let inner = &group[1..group.len() - 1];
      let mask = if inner.is_empty() {
        0
      } else {
        inner
          .split(',')
          .fold(0u16, |acc, s| acc | (1 << s.parse::<u8>().unwrap()))
      };

      button_masks[num_buttons as usize] = mask;
      num_buttons += 1;
    }

    Machine { num_lights, num_buttons, button_masks, target_mask }
  }

  fn build_augmented_matrix(m: &Machine, rows: &mut [Row; MAX_LIGHTS]) {
    for (light, row_slot) in
      rows.iter_mut().enumerate().take(m.num_lights as usize)
    {
      let mut row = 0u16;

      // Button coefficients
      for (btn, &mask) in m
        .button_masks
        .iter()
        .take(m.num_buttons as usize)
        .enumerate()
      {
        if (mask >> light) & 1 == 1 {
          row |= 1 << btn;
        }
      }

      // RHS
      if (m.target_mask >> light) & 1 == 1 {
        row |= 1 << m.num_buttons;
      }

      *row_slot = row;
    }
  }

  fn gaussian_elimination(
    rows: &mut [Row; MAX_LIGHTS],
    num_rows: usize,
    num_cols: usize,
  ) -> (u8, [i8; MAX_LIGHTS], bool) {
    // Forward elimination
    let mut pivot_col = [-1i8; MAX_LIGHTS];
    let mut rank = 0u8;

    for col in 0..num_cols {
      if rank as usize >= num_rows {
        break;
      }

      let row = rank as usize;
      let pivot = (row..num_rows).find(|&r| rows[r] & (1 << col) != 0);

      if let Some(pivot) = pivot {
        rows.swap(row, pivot);

        // Eliminate below and above (RREF in one pass)
        for r in 0..num_rows {
          if r != row && rows[r] & (1 << col) != 0 {
            rows[r] ^= rows[row];
          }
        }

        pivot_col[row] = col as i8;
        rank += 1;
      }
    }

    // Check inconsistency
    let rhs_bit = num_cols;
    let inconsistent = (0..num_rows).any(|r| {
      rows[r] & ((1 << num_cols) - 1) == 0 && (rows[r] >> rhs_bit) & 1 == 1
    });

    (rank, pivot_col, inconsistent)
  }

  fn solve_linear_system(
    rows: &[Row; MAX_LIGHTS],
    pivot_col: &[i8; MAX_LIGHTS],
    rank: u8,
    num_buttons: usize,
  ) -> u32 {
    let rhs_bit = num_buttons;

    // Extract particular solution
    let particular = (0..rank as usize)
      .filter(|&r| (rows[r] >> rhs_bit) & 1 == 1)
      .fold(0u16, |acc, r| acc | (1 << pivot_col[r]));

    // Find free variables and build nullspace
    let pivot_mask =
      (0..rank as usize).fold(0u16, |acc, r| acc | (1 << pivot_col[r]));

    let mut nullspace = [0u16; MAX_BUTTONS];
    let mut num_free = 0;

    for col in 0..num_buttons {
      if pivot_mask & (1 << col) == 0 {
        let mut basis_vec = 1u16 << col;

        for r in 0..rank as usize {
          if (rows[r] & (1 << col)) != 0 {
            basis_vec |= 1 << pivot_col[r];
          }
        }

        nullspace[num_free] = basis_vec;
        num_free += 1;
      }
    }

    // Find minimum weight solution
    if num_free == 0 {
      return particular.count_ones();
    }

    (0..1u32 << num_free)
      .map(|mask| {
        let mut sol = particular;

        for (i, &basis) in nullspace.iter().take(num_free).enumerate() {
          if mask & (1 << i) != 0 {
            sol ^= basis;
          }
        }

        sol.count_ones()
      })
      .min()
      .unwrap()
  }

  pub fn part1(p: &Problem) -> u64 {
    let mut total = 0u64;

    for line in p.input.lines() {
      let trimmed = line.trim();
      if trimmed.is_empty() {
        continue;
      }

      let machine = parse_machine(trimmed);
      let mut rows = [0u16; MAX_LIGHTS];

      build_augmented_matrix(&machine, &mut rows);

      let (rank, pivot_col, inconsistent) = gaussian_elimination(
        &mut rows,
        machine.num_lights as usize,
        machine.num_buttons as usize,
      );

      if inconsistent {
        total += u32::MAX as u64;
      } else {
        let min_presses = solve_linear_system(
          &rows,
          &pivot_col,
          rank,
          machine.num_buttons as usize,
        );
        total += min_presses as u64;
      }
    }

    total
  }
}

#[cfg(feature = "part2")]
mod part2_impl {
  use super::{MAX_BUTTONS, MAX_LIGHTS, Problem};

  struct Machine {
    num_buttons: usize,
    num_counters: usize,
    button_masks: [u16; MAX_BUTTONS],
    target: [i64; MAX_LIGHTS],
  }

  fn parse_machine(line: &str) -> Machine {
    let mut button_masks = [0u16; MAX_BUTTONS];
    let mut target = [0i64; MAX_LIGHTS];
    let (mut num_buttons, mut num_counters) = (0, 0);
    for tok in line.split_whitespace() {
      if tok.starts_with('(') {
        let inner = &tok[1..tok.len() - 1];
        let mut mask = 0u16;
        if !inner.is_empty() {
          for p in inner.split(',') {
            mask |= 1 << p.parse::<u8>().unwrap();
          }
        }
        button_masks[num_buttons] = mask;
        num_buttons += 1;
      } else if tok.starts_with('{') {
        let inner = &tok[1..tok.len() - 1];
        for (i, s) in inner.split(',').enumerate() {
          target[i] = s.parse().unwrap();
          num_counters = i + 1;
        }
      }
    }
    Machine { num_buttons, num_counters, button_masks, target }
  }

  fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
      let t = b;
      b = a % b;
      a = t;
    }
    if a == 0 { 1 } else { a }
  }

  fn reduce_row(
    a: &mut [[i64; MAX_BUTTONS]; MAX_LIGHTS],
    b: &mut [i64],
    r: usize,
    cols: usize,
  ) {
    let mut g = if b[r] != 0 { b[r].abs() } else { 0 };

    for &val in a[r].iter().take(cols) {
      if val != 0 {
        let abs = val.abs();
        g = if g == 0 { abs } else { gcd(g, abs) };
      }
    }

    if g > 1 {
      for val in a[r].iter_mut().take(cols) {
        *val /= g;
      }
      b[r] /= g;
    }
  }

  fn eliminate_rref(
    a: &mut [[i64; MAX_BUTTONS]; MAX_LIGHTS],
    b: &mut [i64; MAX_LIGHTS],
    rows: usize,
    cols: usize,
  ) -> (usize, [i32; MAX_LIGHTS], bool) {
    let mut pivot_col = [-1i32; MAX_LIGHTS];
    let mut rank = 0;

    for c in 0..cols {
      if rank >= rows {
        break;
      }

      // Find pivot row (case 1: ±1 pivot)
      let mut pr = None;
      for (r, row) in a.iter().enumerate().take(rows).skip(rank) {
        if row[c] == 1 || row[c] == -1 {
          pr = Some(r);
          break;
        }
      }

      // Fallback pivot row (any non-zero)
      if pr.is_none() {
        for (r, row) in a.iter().enumerate().take(rows).skip(rank) {
          if row[c] != 0 {
            pr = Some(r);
            break;
          }
        }
      }

      let Some(pivot_row) = pr else {
        continue;
      };

      a.swap(rank, pivot_row);
      b.swap(rank, pivot_row);

      // Normalize sign
      if a[rank][c] < 0 {
        for val in a[rank].iter_mut().take(cols) {
          *val = -*val;
        }
        b[rank] = -b[rank];
      }

      let pivot = a[rank][c];

      // Eliminate r ≠ rank
      for r in 0..rows {
        if r == rank {
          continue;
        }

        let factor = a[r][c];
        if factor == 0 {
          continue;
        }

        // Borrow r and rank rows without overlapping
        let (row_r, row_rank) = if r < rank {
          let (top, bottom) = a.split_at_mut(rank);
          (&mut top[r], &bottom[0])
        } else {
          let (top, bottom) = a.split_at_mut(r);
          (&mut bottom[0], &top[rank])
        };

        // Now row_r and row_rank are safely alias-free
        for (dst, src) in
          row_r.iter_mut().take(cols).zip(row_rank.iter().take(cols))
        {
          *dst = pivot * *dst - factor * *src;
        }

        b[r] = pivot * b[r] - factor * b[rank];
        reduce_row(a, b, r, cols);
      }

      pivot_col[rank] = c as i32;
      rank += 1;
    }

    // Check inconsistency
    for r in rank..rows {
      let all_zero = a[r].iter().take(cols).all(|&x| x == 0);
      if all_zero && b[r] != 0 {
        return (rank, pivot_col, true);
      }
    }

    (rank, pivot_col, false)
  }

  fn back_sub(
    a: &[[i64; MAX_BUTTONS]; MAX_LIGHTS],
    b: &[i64; MAX_LIGHTS],
    cols: usize,
    pivot_col: &[i32; MAX_LIGHTS],
    rank: usize,
    is_pivot: &[bool; MAX_BUTTONS],
    x: &mut [i64; MAX_BUTTONS],
  ) -> Option<i64> {
    let mut sum = 0i64;
    for r in 0..rank {
      let pc = pivot_col[r] as usize;
      let coeff = a[r][pc];
      let mut sf = 0i64;
      for c in 0..cols {
        if !is_pivot[c] {
          sf += a[r][c] * x[c];
        }
      }
      let rhs = b[r] - sf;
      if rhs % coeff != 0 || rhs / coeff < 0 {
        return None;
      }
      let val = rhs / coeff;
      x[pc] = val;
      sum += val;
    }
    Some(sum)
  }

  fn solve(m: &Machine) -> u32 {
    let (rows, cols) = (m.num_counters, m.num_buttons);
    let mut a = [[0i64; MAX_BUTTONS]; MAX_LIGHTS];
    let mut b = [0i64; MAX_LIGHTS];

    for r in 0..rows {
      b[r] = m.target[r];
      for (c, val) in a[r].iter_mut().enumerate().take(cols) {
        *val = ((m.button_masks[c] >> r) & 1) as i64;
      }
    }

    let (rank, pivot_col, bad) = eliminate_rref(&mut a, &mut b, rows, cols);
    if bad {
      return u32::MAX;
    }

    let mut is_pivot = [false; MAX_BUTTONS];
    for r in 0..rank {
      if pivot_col[r] >= 0 {
        is_pivot[pivot_col[r] as usize] = true;
      }
    }

    let mut free = [0usize; MAX_BUTTONS];
    let mut nf = 0;

    for (c, &is_p) in is_pivot.iter().enumerate().take(cols) {
      if !is_p {
        free[nf] = c;
        nf += 1;
      }
    }

    let max_t = (0..rows).map(|r| m.target[r]).max().unwrap_or(0);

    let mut best = u32::MAX;
    let mut x = [0i64; MAX_BUTTONS];

    struct DfsCtx<'a> {
      free: &'a [usize],
      max_v: i64,
      a: &'a [[i64; MAX_BUTTONS]; MAX_LIGHTS],
      b: &'a [i64; MAX_LIGHTS],
      cols: usize,
      pivot_col: &'a [i32; MAX_LIGHTS],
      rank: usize,
      is_pivot: &'a [bool; MAX_BUTTONS],
      best: &'a mut u32,
      x: &'a mut [i64; MAX_BUTTONS],
    }

    fn dfs(j: usize, ps: i64, ctx: &mut DfsCtx<'_>) {
      if j == ctx.free.len() {
        if let Some(s) = back_sub(
          ctx.a,
          ctx.b,
          ctx.cols,
          ctx.pivot_col,
          ctx.rank,
          ctx.is_pivot,
          ctx.x,
        ) {
          let t = ps + s;
          if t >= 0 && (t as u32) < *ctx.best {
            *ctx.best = t as u32;
          }
        }
        return;
      }

      let col = ctx.free[j];

      for v in 0..=ctx.max_v {
        let ns = ps + v;
        if ns >= *ctx.best as i64 {
          break;
        }

        ctx.x[col] = v;
        dfs(j + 1, ns, ctx);
      }

      ctx.x[col] = 0;
    }

    let free_slice = &free[..nf];

    let mut ctx = DfsCtx {
      free: free_slice,
      max_v: max_t,
      a: &a,
      b: &b,
      cols,
      pivot_col: &pivot_col,
      rank,
      is_pivot: &is_pivot,
      best: &mut best,
      x: &mut x,
    };

    dfs(0, 0, &mut ctx);

    best
  }

  pub fn part2(p: &Problem) -> u64 {
    p.input
      .lines()
      .filter(|l| !l.trim().is_empty())
      .map(|l| {
        let r = solve(&parse_machine(l.trim()));
        if r == u32::MAX { 0 } else { r as u64 }
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
    assert_eq!(result, 7);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part2(&problem);
    assert_eq!(result, 33);
  }
}
