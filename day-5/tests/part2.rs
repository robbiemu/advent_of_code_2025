#[cfg(feature = "part2")]
use day_5::prelude::*;

#[cfg(feature = "part2")]
mod std_parse {
  // Reuse the std-only parser inside tests
  include!("../src/std_parse.rs");
}
#[cfg(feature = "part2")]
use std_parse::parse_std;

#[test]
#[cfg(feature = "part2")]
fn test_part2() {
  let input = include_str!("../sample.txt");
  let (mut ranges, ingredients) = parse_std(input);

  // ingredients are irrelevant in part2
  let result = part2(&mut ranges, &ingredients);

  assert_eq!(result, 14);
}
