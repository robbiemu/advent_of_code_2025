use day_4::prelude::*;
#[cfg(feature = "part2")]
use day_4::tinysetqueue::prelude::*;
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
  let problem = parse(black_box(INPUT));
  let total = problem.width * problem.height;

  // allocate caller-managed buffers once per run to reflect the no_alloc API
  let mut present = vec![false; total];
  let mut degree = vec![0u8; total];
  let mut queue_buf = vec![0usize; total];
  let mut in_queue = vec![false; total];

  let mut queue =
    TinySetQueue::new(&mut queue_buf, &mut in_queue, MembershipMode::InQueue);

  black_box(part2(&problem, &mut present, &mut degree, &mut queue));
}

fn main() {
  divan::main();
}
