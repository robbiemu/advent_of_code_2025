use day_7::prelude::*;
mod std_parse;
use std_parse::parse;


#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");


fn main() {
  let mut problem_data = parse(INPUT);
  let mut problem = problem_data.as_problem();

  #[cfg(not(feature = "part2"))]
  {
    let p1 = part1(&mut problem);
    println!("Part 1: {p1}");
  }

  #[cfg(feature = "part2")]
  {
    let p2 = part2(&mut problem);
    println!("Part 2: {p2}");
  }
}
