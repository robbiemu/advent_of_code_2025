#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

use heapless::{Vec, index_map::FnvIndexMap};


pub mod prelude {
  pub use crate::{Problem, parse};

  #[cfg(not(feature = "part2"))]
  pub use crate::part1_impl::part1;

  #[cfg(feature = "part2")]
  pub use crate::part2_impl::part2;
}


// --------------------------
// Data Model
// --------------------------

#[cfg(feature = "sample")]
const C: usize = 3;
#[cfg(feature = "sample")]
const M: usize = 16;
#[cfg(not(feature = "sample"))]
const C: usize = 23;
#[cfg(not(feature = "sample"))]
const M: usize = 1024;

pub struct Problem<'a> {
  nodes: Nodes<'a>,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Node<'a> {
  id: usize,
  name: &'a str,
  visited: bool,
  children: Vec<usize, C>,
}

#[cfg(feature = "std")]
impl std::fmt::Debug for Node<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.visited {
      write!(f, "Node{{ {}, visited }}", self.name)
    } else {
      write!(f, "Node{{ {} }}", self.name)
    }
  }
}

type Nodes<'a> = FnvIndexMap<usize, Node<'a>, M>;


fn id_from_str(s: &str) -> usize {
  let mut n: u64 = 0;
  for &b in s.as_bytes() {
    n = (n << 8) | (b as u64);
  }
  n as usize
}


// --------------------------
// Parse
// --------------------------

pub fn parse<'a>(input: &'a str) -> Problem<'a> {
  let mut nodes: Nodes<'a> = FnvIndexMap::new();

  for line in input.lines() {
    let (mut parent_str, descendents_str) =
      line.split_once(":").unwrap_or((line, ""));

    parent_str = parent_str.trim();

    let pid = id_from_str(parent_str);
    let _ = nodes.entry(pid).or_insert(Node {
      id: pid,
      name: parent_str,
      visited: false,
      children: Vec::new(),
    });

    for child in descendents_str.split_whitespace() {
      let child = child.trim();

      let id = id_from_str(child);
      let _ = nodes.entry(id).or_insert(Node {
        id,
        name: child,
        visited: false,
        children: Vec::new(),
      });

      let _ = nodes.get_mut(&pid).unwrap().children.push(id);
    }
  }

  Problem { nodes }
}


// --------------------------
// Solver — Part 1
// --------------------------
#[cfg(not(feature = "part2"))]
mod part1_impl {
  use super::{Nodes, Problem};

  fn dfs<'a>(idx: usize, out: usize, nodes: &mut Nodes<'a>, count: &mut usize) {
    if idx == out {
      *count += 1;
      return;
    }

    nodes[&idx].visited = true;

    let children = nodes[&idx].children.clone();
    for child in children.iter() {
      if !nodes[child].visited {
        dfs(*child, out, nodes, count);
      }
    }

    nodes[&idx].visited = false;
  }

  pub fn part1(p: &Problem) -> usize {
    let you = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "you")
      .map(|(id, _)| *id)
      .unwrap();
    let out = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "out")
      .map(|(id, _)| *id)
      .unwrap();

    let mut count = 0;
    dfs(you, out, &mut p.nodes.clone(), &mut count);

    count
  }
}


// --------------------------
// Solver — Part 2
// --------------------------

#[cfg(feature = "part2")]
mod part2_impl {
  use heapless::index_map::FnvIndexMap;

  use super::{M, Nodes, Problem};


  fn dfs<'a>(
    idx: usize,
    target: usize,
    nodes: &mut Nodes<'a>,
    memo: &mut FnvIndexMap<usize, usize, M>,
  ) -> usize {
    if idx == target {
      return 1;
    }

    if let Some(&cached) = memo.get(&idx) {
      return cached;
    }

    nodes[&idx].visited = true;

    let mut count = 0;
    let children = nodes[&idx].children.clone();
    for child in children.iter() {
      if !nodes[child].visited {
        count += dfs(*child, target, nodes, memo);
      }
    }

    nodes[&idx].visited = false;
    memo.insert(idx, count).ok();

    count
  }

  pub fn part2(p: &Problem) -> usize {
    let svr = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "svr")
      .map(|(id, _)| *id)
      .unwrap();
    let dac = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "dac")
      .map(|(id, _)| *id)
      .unwrap();
    let fft = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "fft")
      .map(|(id, _)| *id)
      .unwrap();
    let out = p
      .nodes
      .iter()
      .find(|(_, n)| n.name == "out")
      .map(|(id, _)| *id)
      .unwrap();


    // Compute each unique segment once
    let mut memo = FnvIndexMap::new();
    let svr_to_dac = dfs(svr, dac, &mut p.nodes.clone(), &mut memo);

    let mut memo = FnvIndexMap::new();
    let svr_to_fft = dfs(svr, fft, &mut p.nodes.clone(), &mut memo);

    let mut memo = FnvIndexMap::new();
    let dac_to_fft = dfs(dac, fft, &mut p.nodes.clone(), &mut memo);

    let mut memo = FnvIndexMap::new();
    let fft_to_dac = dfs(fft, dac, &mut p.nodes.clone(), &mut memo);

    let mut memo = FnvIndexMap::new();
    let dac_to_out = dfs(dac, out, &mut p.nodes.clone(), &mut memo);

    let mut memo = FnvIndexMap::new();
    let fft_to_out = dfs(fft, out, &mut p.nodes.clone(), &mut memo);


    #[cfg(feature = "std")]
    {
      println!(
        "svr->dac: {}, svr->fft: {}, dac->fft: {}, fft->dac: {}, dac->out: \
         {}, fft->out: {}",
        svr_to_dac, svr_to_fft, dac_to_fft, fft_to_dac, dac_to_out, fft_to_out
      );
    }
    // Combine segments
    let paths_dac_first = svr_to_dac * dac_to_fft * fft_to_out;
    let paths_fft_first = svr_to_fft * fft_to_dac * dac_to_out;

    paths_dac_first + paths_fft_first
  }
}


#[cfg(test)]
mod tests {
  use super::prelude::*;

  #[test]
  #[cfg(not(feature = "part2"))]
  fn test_part1() {
    let input = include_str!("../sample.txt");
    let problem = parse(input);
    let result = part1(&problem);

    assert_eq!(result, 5);
  }

  #[test]
  #[cfg(feature = "part2")]
  fn test_part2() {
    let input = include_str!("../sample.part2.txt");
    let problem = parse(input);
    let result = part2(&problem);
    assert_eq!(result, 2);
  }
}
