use day_11::prelude::*;


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
    let p2 = part2(&problem);
    println!("Part 2: {p2}");
  }
}
