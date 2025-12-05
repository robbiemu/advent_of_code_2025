use day_5::prelude::*;

mod std_parse;

use std_parse::parse_std;

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

fn main() {
  // std_parse does the std-only parsing for ranges + ingredients
  let (mut ranges, ingredients) = parse_std(INPUT);

  #[cfg(not(feature = "part2"))]
  {
    let p1 = part1(&mut ranges, &ingredients);
    println!("Part 1: {p1}");
  }

  #[cfg(feature = "part2")]
  {
    let p2 = part2(&mut ranges, &ingredients);
    println!("Part 2: {p2}");
  }
}
