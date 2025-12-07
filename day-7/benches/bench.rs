use day_7::prelude::*;
use divan::black_box;

mod std_parse {
  include!("../src/std_parse.rs");
}

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
#[divan::bench]
fn bench_part1() {
  let mut problem_data = std_parse::parse(black_box(INPUT));
  let mut p = problem_data.as_problem();
  black_box(part1(&mut p));
}

#[cfg(feature = "part2")]
#[divan::bench]
fn bench_part2() {
  let mut problem_data = std_parse::parse(black_box(INPUT));
  let mut p = problem_data.as_problem();
  black_box(part2(&mut p));
}

fn main() {
  divan::main();
}
