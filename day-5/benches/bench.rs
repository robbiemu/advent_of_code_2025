use day_5::prelude::*;
use divan::black_box;

mod std_parse {
  include!("../src/std_parse.rs");
}
use std_parse::parse_std;

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
#[divan::bench]
fn bench_part1() {
  // Parse fresh each time because part1 MUTATES the ranges array
  let (mut ranges, ingredients) = parse_std(black_box(INPUT));

  black_box(part1(&mut ranges, &ingredients));
}

#[cfg(feature = "part2")]
#[divan::bench]
fn bench_part2() {
  let (mut ranges, ingredients) = parse_std(black_box(INPUT));

  black_box(part2(&mut ranges, &ingredients));
}

fn main() {
  divan::main();
}
