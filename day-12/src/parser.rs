use heapless::Vec;
use nom::{
  IResult, Parser,
  bytes::complete::take_while1,
  character::complete::{char, digit1, line_ending, space0, space1},
  combinator::{map_res, opt},
  multi::fold_many0,
  sequence::terminated,
};

use crate::data_model::{MAX_REGIONS, Region, SHAPE_H, SHAPE_W, SHAPES, Shape};


pub mod prelude {
  #[allow(unused_imports)]
  pub use super::{LineBytes, parse_regions, parse_shapes, skip_blank_lines};
}

// -----------------------------
// Shapes
// -----------------------------

pub fn parse_shapes(i: &str) -> IResult<&str, Vec<Shape, SHAPES>> {
  let (i, _) = skip_blank_lines(i)?;
  fold_many0(
    terminated(parse_shape_block, skip_blank_lines),
    Vec::<Shape, SHAPES>::new,
    |mut acc, s| {
      let _ = acc.push(s);
      acc
    },
  )
  .parse(i)
}

fn parse_shape_block(i: &str) -> IResult<&str, Shape> {
  // "<id>:\n"
  let (i, id) = u8_num(i)?;
  let (i, _) = (char(':'), line_ending).parse(i)?;

  let mut lines: Vec<LineBytes, SHAPE_H> = Vec::new();
  let mut cur = i;

  loop {
    if cur.is_empty() || is_blank_line(cur) {
      break;
    }

    let (rest, row) = shape_row(cur)?;
    let (rest, _) = opt(line_ending).parse(rest)?;
    cur = rest;

    let _ = lines.push(row);
  }

  if lines.is_empty() {
    return Err(nom::Err::Error(nom::error::Error::new(
      cur,
      nom::error::ErrorKind::Many1,
    )));
  }

  let h = lines.len();
  let w = lines[0].len();

  for r in &lines {
    if r.len() != w {
      return Err(nom::Err::Failure(nom::error::Error::new(
        cur,
        nom::error::ErrorKind::Verify,
      )));
    }
  }

  if w == 0 || h == 0 || w > SHAPE_W || h > SHAPE_H {
    return Err(nom::Err::Failure(nom::error::Error::new(
      cur,
      nom::error::ErrorKind::TooLarge,
    )));
  }

  let mut mask = 0u64;
  for (y, row) in lines.iter().enumerate() {
    for (x, b) in row.iter().enumerate() {
      if *b == b'#' {
        let bit = (y * w + x) as u64;
        mask |= 1u64 << bit;
      }
    }
  }

  let area = mask.count_ones() as u8;

  let shape = Shape {
    id,
    w: w as u8,
    h: h as u8,
    mask,
    area,
    tight_pair_area: None, // temporary
  };

  let tight = detect_tight_pair_area(&shape);

  Ok((cur, Shape { tight_pair_area: tight, ..shape }))
}

#[derive(Clone, Copy)]
struct OrientedMask {
  mask: u64,
  w: usize,
  h: usize,
}

fn compute_oriented_masks(shape: &Shape) -> [OrientedMask; 8] {
  let w = shape.w as usize;
  let h = shape.h as usize;

  let bit =
    |m: u64, x: usize, y: usize, mw: usize| ((m >> (y * mw + x)) & 1) != 0;

  let mut out = [OrientedMask { mask: 0, w: 0, h: 0 }; 8];

  // R0
  out[0] = OrientedMask { mask: shape.mask, w, h };

  // R90
  {
    let mut m = 0;
    for y in 0..h {
      for x in 0..w {
        if bit(shape.mask, x, y, w) {
          let nx = h - 1 - y;
          let ny = x;
          m |= 1 << (ny * h + nx);
        }
      }
    }
    out[1] = OrientedMask { mask: m, w: h, h: w };
  }

  // R180
  {
    let mut m = 0;
    for y in 0..h {
      for x in 0..w {
        if bit(shape.mask, x, y, w) {
          let nx = w - 1 - x;
          let ny = h - 1 - y;
          m |= 1 << (ny * w + nx);
        }
      }
    }
    out[2] = OrientedMask { mask: m, w, h };
  }

  // R270
  {
    let mut m = 0;
    for y in 0..h {
      for x in 0..w {
        if bit(shape.mask, x, y, w) {
          let nx = y;
          let ny = w - 1 - x;
          m |= 1 << (ny * h + nx);
        }
      }
    }
    out[3] = OrientedMask { mask: m, w: h, h: w };
  }

  // mirrors
  for i in 0..4 {
    let base = out[i];
    let mut m = 0;
    for y in 0..base.h {
      for x in 0..base.w {
        if bit(base.mask, x, y, base.w) {
          let nx = base.w - 1 - x;
          m |= 1 << (y * base.w + nx);
        }
      }
    }
    out[4 + i] = OrientedMask { mask: m, w: base.w, h: base.h };
  }

  out
}

fn detect_tight_pair_area(shape: &Shape) -> Option<usize> {
  let tiles = shape.mask.count_ones() as usize;
  let tiles2 = 2 * tiles;

  let ref_area = core::cmp::min(
    shape.h as usize * (2 * shape.w as usize),
    (2 * shape.h as usize) * shape.w as usize,
  );
  let ref_waste = ref_area.saturating_sub(tiles2);

  let orients = compute_oriented_masks(shape);
  let mut best: Option<usize> = None;

  for a in &orients {
    for b in &orients {
      for dy in -3..=3 {
        for dx in -3..=3 {
          let mut overlap = false;

          for y in 0..a.h {
            for x in 0..a.w {
              if ((a.mask >> (y * a.w + x)) & 1) == 0 {
                continue;
              }

              let bx = x as isize - dx;
              let by = y as isize - dy;

              if bx >= 0
                && by >= 0
                && (bx as usize) < b.w
                && (by as usize) < b.h
                && ((b.mask >> (by as usize * b.w + bx as usize)) & 1) != 0
              {
                overlap = true;
                break;
              }
            }
            if overlap {
              break;
            }
          }

          if overlap {
            continue;
          }

          let min_x = 0.min(dx);
          let min_y = 0.min(dy);
          let max_x = (a.w as isize).max(dx + b.w as isize);
          let max_y = (a.h as isize).max(dy + b.h as isize);

          let area = (max_x - min_x) as usize * (max_y - min_y) as usize;
          let waste = area.saturating_sub(tiles2);

          if waste < ref_waste {
            best = Some(best.map_or(area, |v| v.min(area)));
          }
        }
      }
    }
  }

  best
}

// -----------------------------
// Shape rows
// -----------------------------

pub type LineBytes = heapless::Vec<u8, SHAPE_W>;

pub fn shape_row(i: &str) -> IResult<&str, LineBytes> {
  let (rest, s) = take_while1(|c| c == '.' || c == '#')(i)?;
  let mut v = LineBytes::new();
  for &b in s.as_bytes() {
    let _ = v.push(b);
  }
  Ok((rest, v))
}

// -----------------------------
// Blank lines
// -----------------------------

pub fn skip_blank_lines(i: &str) -> IResult<&str, ()> {
  fold_many0(blank_line, || (), |_, _| ())
    .parse(i)
    .map(|(r, _)| (r, ()))
}

pub fn blank_line(i: &str) -> IResult<&str, ()> {
  let (i, _) = space0.parse(i)?;
  let (i, _) = line_ending.parse(i)?;
  Ok((i, ()))
}

pub fn is_blank_line(i: &str) -> bool {
  let bytes = i.as_bytes();
  let mut p = 0;
  while p < bytes.len()
    && (bytes[p] == b' ' || bytes[p] == b'\t' || bytes[p] == b'\r')
  {
    p += 1;
  }
  p < bytes.len() && bytes[p] == b'\n'
}

// -----------------------------
// Regions
// -----------------------------

pub fn parse_regions(i: &str) -> IResult<&str, Vec<Region, MAX_REGIONS>> {
  fold_many0(
    terminated(region_line, opt(line_ending)),
    Vec::<Region, MAX_REGIONS>::new,
    |mut acc, r| {
      let _ = acc.push(r);
      acc
    },
  )
  .parse(i)
}

pub fn region_line(i: &str) -> IResult<&str, Region> {
  let (i, w) = u16_num(i)?;
  let (i, _) = char('x').parse(i)?;
  let (i, h) = u16_num(i)?;
  let (i, _) = (char(':'), space0).parse(i)?;

  let mut cur = i;
  let mut counts = [0u16; SHAPES];

  for (idx, slot) in counts.iter_mut().enumerate() {
    let (i2, n) = u16_num(cur)?;
    *slot = n;

    if idx + 1 < SHAPES {
      let (i3, _) = space1.parse(i2)?;
      cur = i3;
    } else {
      cur = i2;
    }
  }

  Ok((cur, Region { w, h, counts }))
}

// -----------------------------
// Numbers
// -----------------------------

pub fn u8_num(i: &str) -> IResult<&str, u8> {
  map_res(digit1, |s: &str| s.parse::<u8>()).parse(i)
}

pub fn u16_num(i: &str) -> IResult<&str, u16> {
  map_res(digit1, |s: &str| s.parse::<u16>()).parse(i)
}
