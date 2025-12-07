use day_6::prelude::*;
use divan::black_box;

mod std_parse {
  include!("../src/std_parse.rs");
}
use std_parse::parse;

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
#[divan::bench]
fn bench_part1() {
  let problem = parse(black_box(INPUT));

  if let Some(first_row) = problem.rows.first() {
    let mut accumulator = first_row.clone();

    part1(&problem.rows[1..], &problem.operands, &mut accumulator);

    black_box(accumulator.into_iter().sum::<u64>());
  } else {
    black_box(0u64);
  }
}

#[cfg(feature = "part2")]
#[divan::bench]
fn bench_part2() {
  let problem = parse(black_box(INPUT));

  if !problem.rows.is_empty() {
    #[allow(nonstandard_style)]
    let (W, G, R) = problem.get_parameters();

    let mut groups_cols = vec![vec![0usize; W]; G];
    let mut groups_lens = vec![0usize; G];
    let mut current_cols = vec![0usize; W];
    let mut problem_row_vals = vec![0u64; R];

    let mut groups_cols_refs: Vec<&mut [usize]> =
      groups_cols.iter_mut().map(|v| v.as_mut_slice()).collect();

    let total = part2(
      &problem.rows[..],
      &problem.operands,
      &mut groups_cols_refs,
      &mut groups_lens,
      &mut current_cols,
      &mut problem_row_vals,
    );

    black_box(total);
  } else {
    black_box(0u64);
  }
}

fn main() {
  divan::main();
}
