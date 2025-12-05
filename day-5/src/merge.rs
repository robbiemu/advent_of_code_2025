pub fn merge_intervals(ranges: &mut [(u64, u64)]) -> usize {
  if ranges.is_empty() {
    return 0;
  }

  ranges.sort_unstable_by_key(|&(start, _)| start);

  let mut out = 0;
  let (mut current_start, mut current_end) = ranges[0];

  for i in 1..ranges.len() {
    let (start, end) = ranges[i];

    if start <= current_end.saturating_add(1) {
      current_end = current_end.max(end);
    } else {
      ranges[out] = (
        current_start,
        current_end.checked_add(1).expect("interval end overflow"),
      );
      out += 1;
      current_start = start;
      current_end = end;
    }
  }

  ranges[out] = (
    current_start,
    current_end.checked_add(1).expect("interval end overflow"),
  );

  out + 1
}
