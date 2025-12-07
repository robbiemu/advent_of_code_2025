#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
  pub use crate::Operand;

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum Operand {
  Multiplication,
  Division,
  Addition,
  Subtraction,
}

// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::Operand;

  fn apply_formula<R>(rows: &[R], operands: &[Operand], accumulator: &mut [u64])
  where
    R: AsRef<[u64]>,
  {
    for row in rows {
      let iter = accumulator
        .iter_mut()
        .zip(row.as_ref().iter())
        .zip(operands.iter());

      for ((acc, &val), op) in iter {
        match op {
          Operand::Multiplication => *acc *= val,
          Operand::Division => *acc /= val,
          Operand::Addition => *acc += val,
          Operand::Subtraction => *acc -= val,
        }
      }
    }
  }

  // applies math operations, so works in pairs startings from the row 0 value in the accumulator
  pub fn part1<R>(rows: &[R], operands: &[Operand], accumulator: &mut [u64])
  where
    R: AsRef<[u64]>,
  {
    apply_formula(rows, operands, accumulator);
  }
}

// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use super::Operand;


  pub fn part2<S: AsRef<str>>(
    rows: &[S],
    operands: &[Operand],

    groups_cols: &mut [&mut [usize]], // caller-provided 2D buffer
    groups_lens: &mut [usize],        // lengths for each group
    current_cols: &mut [usize],       // scratch buffer for 1 group
    problem_row_vals: &mut [u64],     // scratch row buffer
  ) -> u64 {
    #[allow(nonstandard_style)]
    let W = rows[0].as_ref().len();
    #[allow(nonstandard_style)]
    let R = rows.len();

    let mut groups_count = 0usize;
    let mut current_len = 0usize;

    for c in 0..W {
      let column_empty = (0..R).all(|r| rows[r].as_ref().as_bytes()[c] == b' ');

      if column_empty {
        // finished building a group, commit it
        if current_len > 0 {
          groups_cols[groups_count][..current_len]
            .copy_from_slice(&current_cols[..current_len]);

          groups_lens[groups_count] = current_len;
          groups_count += 1;
          current_len = 0;
        }
      } else {
        current_cols[current_len] = c;
        current_len += 1;
      }
    }

    if current_len > 0 {
      groups_cols[groups_count][..current_len]
        .copy_from_slice(&current_cols[..current_len]);

      groups_lens[groups_count] = current_len;
      groups_count += 1;
    }

    let mut total = 0u64;

    for group_index in 0..groups_count {
      let cols_len = groups_lens[group_index];
      let op = &operands[group_index];

      for k in 0..cols_len {
        let c = groups_cols[group_index][k];
        let mut value = 0u64;

        for row in rows.iter() {
          let ch = row.as_ref().as_bytes()[c];
          if ch.is_ascii_digit() {
            value = value * 10 + (ch - b'0') as u64;
          }
        }

        problem_row_vals[k] = value;
      }

      #[cfg(feature = "std")]
      {
        eprintln!(
          "GROUP {} (op {:?}): {:?}",
          group_index,
          op,
          &problem_row_vals[..cols_len]
        );
      }

      let mut group_value = problem_row_vals[0];
      for &v in &problem_row_vals[1..cols_len] {
        group_value = match op {
          Operand::Addition => group_value + v,
          Operand::Subtraction => group_value - v,
          Operand::Multiplication => group_value * v,
          Operand::Division => group_value / v,
        };
      }

      total += group_value;
    }

    total
  }
}
