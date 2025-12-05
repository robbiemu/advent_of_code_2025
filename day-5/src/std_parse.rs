/// This stays in std-land and never contaminates the no_std logic.
pub fn parse_std(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
  let mut ranges = Vec::new();
  let mut ingredients = Vec::new();

  let mut lines = input.lines();

  // parse ranges until blank line
  for line in &mut lines {
    let line = line.trim();
    if line.is_empty() {
      break;
    }
    // format is like "3-5"
    let (a, b) = line.split_once('-').expect("invalid range");
    ranges.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
  }

  // remaining lines = ingredients
  for line in lines {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }
    ingredients.push(line.parse::<u64>().unwrap());
  }

  (ranges, ingredients)
}
