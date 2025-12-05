pub fn merge_intervals(ranges: &mut [(u64, u64)]) -> usize {
  for (_, end) in ranges.iter_mut() {
    *end = end.checked_add(1).expect("interval end overflow");
  }

  ranges.sort_by_key(|&(start, _)| start);

  let mut out = 0;
  let (mut current_start, mut current_end) = ranges[0];
  for i in 1..ranges.len() {
    let (start, end) = ranges[i];

    if start <= current_end {
      // extend
      current_end = current_end.max(end);
    } else {
      // flush previous interval
      ranges[out] = (current_start, current_end);
      out += 1;
      current_start = start;
      current_end = end;
    }
  }
  ranges[out] = (current_start, current_end);


  out + 1
}
