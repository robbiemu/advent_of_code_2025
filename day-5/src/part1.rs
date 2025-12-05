#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use crate::prelude::RangeSearch;
use crate::prelude::merge_intervals;


pub fn part1(ranges: &mut [(u64, u64)], ingredients: &[u64]) -> usize {
  let n = merge_intervals(ranges);

  let map = RangeSearch::new(&ranges[..n]);

  ingredients.iter().filter(|&&id| map.contains(id)).count()
}
