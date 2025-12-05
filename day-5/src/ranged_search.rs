#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

use wide::u64x4;

pub struct RangeSearch {
  starts: Vec<u64>,
  ends: Vec<u64>,
}

impl RangeSearch {
  pub fn new(ranges: &[(u64, u64)]) -> Self {
    let mut starts = Vec::with_capacity(ranges.len());
    let mut ends = Vec::with_capacity(ranges.len());

    for &(s, e) in ranges {
      starts.push(s);
      ends.push(e);
    }

    RangeSearch { starts, ends }
  }

  #[inline]
  pub fn contains(&self, id: u64) -> bool {
    const LANES: usize = 4;

    let n = self.starts.len();
    if n == 0 {
      return false;
    }

    let idv = u64x4::splat(id);

    let mut i = 0;
    while i + LANES <= n {
      let s = u64x4::from([
        self.starts[i],
        self.starts[i + 1],
        self.starts[i + 2],
        self.starts[i + 3],
      ]);

      let e = u64x4::from([
        self.ends[i],
        self.ends[i + 1],
        self.ends[i + 2],
        self.ends[i + 3],
      ]);

      // SIMD condition: (id >= start) & (id < end)
      let ge = !idv.simd_lt(s); // id >= s
      let lt = idv.simd_lt(e); // id < e
      let mask = ge & lt;

      let lanes = mask.to_array();
      if lanes[0] != 0 || lanes[1] != 0 || lanes[2] != 0 || lanes[3] != 0 {
        return true;
      }

      i += LANES;
    }

    // Scalar tail for remaining 0–3 intervals
    let mut found: u8 = 0;

    for j in i..n {
      let ge = (id >= self.starts[j]) as u8;
      let lt = (id < self.ends[j]) as u8;
      found |= ge & lt;
    }

    found != 0
  }
}
