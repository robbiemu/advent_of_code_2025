use day_6::prelude::*;

mod std_parse;
use std_parse::parse;


#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");


fn main() {
  let problem = parse(INPUT);

  #[cfg(not(feature = "part2"))]
  {
    if let Some(first_row) = problem.rows.first() {
      let mut results = first_row.clone();

      part1(&problem.rows[1..], &problem.operands, &mut results);

      let p1 = results.into_iter().sum::<u64>();

      println!("Part 1: {p1}");
    } else {
      panic!("No columns found in the problem");
    }
  }

  #[cfg(feature = "part2")]
  {
    if !problem.rows.is_empty() {
      #[allow(nonstandard_style)]
      let (W, G, R) = problem.get_parameters();

      let mut groups_cols = vec![vec![0usize; W]; G];
      let mut groups_lens = vec![0usize; G];
      let mut current_cols = vec![0usize; W];
      let mut problem_row_vals = vec![0u64; R];

      let mut groups_cols_refs: Vec<&mut [usize]> =
        groups_cols.iter_mut().map(|v| v.as_mut_slice()).collect();

      let p2 = part2(
        &problem.rows[..],
        &problem.operands,
        &mut groups_cols_refs,
        &mut groups_lens,
        &mut current_cols,
        &mut problem_row_vals,
      );

      println!("Part 2: {p2}");
    } else {
      panic!("No columns found in the problem");
    }
  }
}
