use day_4::prelude::*;
#[cfg(feature = "part2")]
use day_4::tinysetqueue::prelude::*;

#[cfg(feature = "sample")]
const INPUT: &str = include_str!("../sample.txt");

#[cfg(not(feature = "sample"))]
const INPUT: &str = include_str!("../input.txt");

fn main() {
  let problem = parse(INPUT);

  #[cfg(not(feature = "part2"))]
  {
    let p1 = part1(&problem);
    println!("Part 1: {p1}");
  }

  #[cfg(feature = "part2")]
  {
    // allocate caller-managed working buffers for part2
    let total = problem.width * problem.height;

    // working buffers
    let mut present = vec![false; total];
    let mut degree = vec![0u8; total];

    let mut queue_buf = vec![0usize; total];
    let mut in_queue = vec![false; total]; // ← FIX: must bind to a variable

    let mut queue =
      TinySetQueue::new(&mut queue_buf, &mut in_queue, MembershipMode::InQueue);

    let p2 = part2(&problem, &mut present, &mut degree, &mut queue);
    println!("Part 2: {p2}");
  }
}
