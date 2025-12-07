#[cfg(feature = "part2")]
mod std_parse {
  include!("../src/std_parse.rs");
}
#[cfg(feature = "part2")]
use std_parse::parse;

#[cfg(feature = "part2")]
use day_6::prelude::*;


#[test]
#[cfg(feature = "part2")]
fn test_part2() {
  let input = include_str!("../sample.txt");
  let problem = parse(input);

  if !problem.rows.is_empty() {
    #[allow(nonstandard_style)]
    let (W, G, R) = problem.get_parameters();

    let mut groups_cols = vec![vec![0usize; W]; G];
    let mut groups_lens = vec![0usize; G];
    let mut current_cols = vec![0usize; W];
    let mut problem_row_vals = vec![0u64; R];

    let mut groups_cols_refs: Vec<&mut [usize]> =
      groups_cols.iter_mut().map(|v| v.as_mut_slice()).collect();

    let result = part2(
      &problem.rows[..],
      &problem.operands,
      &mut groups_cols_refs,
      &mut groups_lens,
      &mut current_cols,
      &mut problem_row_vals,
    );

    assert_eq!(result, 3263827);
  } else {
    panic!("No columns found in the problem");
  }
}
