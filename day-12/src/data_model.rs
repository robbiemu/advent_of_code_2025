pub mod prelude {
  #[allow(unused_imports)]
  pub use super::{MAX_REGIONS, Region, SHAPE_H, SHAPE_W, SHAPES, Shape};
}

pub const SHAPES: usize = 6;
pub const MAX_REGIONS: usize = 1000;

pub const SHAPE_W: usize = 3;
pub const SHAPE_H: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
  pub id: u8,
  pub w: u8,
  pub h: u8,
  pub mask: u64,
  pub area: u8,
  pub tight_pair_area: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
  pub w: u16,
  pub h: u16,
  pub counts: [u16; SHAPES],
}
