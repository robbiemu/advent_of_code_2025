#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;


pub fn part2(ranges: &mut [(u64, u64)], _ingredients: &[u64]) -> usize {
  // 1. Merge intervals (same as part1)
  let n = crate::prelude::merge_intervals(ranges);

  // 2. Sum lengths of merged intervals
  let mut total = 0usize;
  for &(s, e) in &ranges[..n] {
    total += (e - s) as usize; // e is exclusive
  }

  total
}
