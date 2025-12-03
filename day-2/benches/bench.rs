use day_2::parse;
#[cfg(not(feature = "part2"))]
use day_2::part1_impl::part1;
#[cfg(feature = "part2")]
use day_2::part2_impl::part2;
use divan::black_box;

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
#[divan::bench]
fn bench_part1() {
  let p = parse(black_box(INPUT));
  black_box(part1(&p));
}

#[cfg(feature = "part2")]
#[divan::bench]
fn bench_part2() {
  let p = parse(black_box(INPUT));
  black_box(part2(&p));
}

fn main() {
  divan::main();
}
