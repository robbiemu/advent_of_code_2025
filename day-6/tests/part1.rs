#[cfg(not(feature = "part2"))]
mod std_parse {
  include!("../src/std_parse.rs");
}
#[cfg(not(feature = "part2"))]
use std_parse::parse;

#[cfg(not(feature = "part2"))]
use day_6::prelude::*;


#[test]
#[cfg(not(feature = "part2"))]
fn test_part1() {
  let input = include_str!("../sample.txt");
  let problem = parse(input);

  if let Some(first_row) = problem.rows.first() {
    let mut results = first_row.clone();

    part1(&problem.rows[1..], &problem.operands, &mut results);

    assert_eq!(results.into_iter().sum::<u64>(), 4277556);
  } else {
    panic!("No columns found in the problem");
  }
}
