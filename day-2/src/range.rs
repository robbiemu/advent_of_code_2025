#![allow(dead_code)]

use crate::u64_handlers::parse_u64_bytes;

/* notes from my teacher chatGPT:

RangeIter is fine, but embedded dev often uses:
- Iterators constructed from &str slices
- No struct fields except a pointer and offset index
- No find() calls (due to code size)
To be appropriate for microcontrollers (smaller).
*/
pub struct RangeIter<'a> {
  input: &'a str,
}

impl<'a> RangeIter<'a> {
  pub fn new(input: &'a str) -> Self {
    Self { input }
  }
}

impl<'a> Iterator for RangeIter<'a> {
  type Item = (u64, u64);

  fn next(&mut self) -> Option<Self::Item> {
    if self.input.is_empty() {
      return None;
    }

    // Find next comma or end
    let (range_str, rest) = match self.input.find(',') {
      Some(idx) => (&self.input[..idx], &self.input[idx + 1..]),
      None => (self.input, ""),
    };

    self.input = rest;

    // Parse "start-end"
    let mut parts = range_str.splitn(2, '-');
    let start = parse_u64_bytes(parts.next()?)?;
    let end = parse_u64_bytes(parts.next()?)?;

    Some((start, end))
  }
}
