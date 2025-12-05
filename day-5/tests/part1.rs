#[cfg(not(feature = "part2"))]
use day_5::prelude::*;

#[cfg(not(feature = "part2"))]
mod std_parse {
  include!("../src/std_parse.rs");
}
#[cfg(not(feature = "part2"))]
use std_parse::parse_std;

#[test]
#[cfg(not(feature = "part2"))]
fn test_part1() {
  let input = include_str!("../sample.txt");
  let (mut ranges, ingredients) = parse_std(input);

  let result = part1(&mut ranges, &ingredients);
  assert_eq!(result, 3);
}
