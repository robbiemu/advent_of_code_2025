#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

const DATA: &str = include_str!("../input.txt");

pub struct ProblemDefinition<'a> {
  pub input: &'a str,
}

pub type Consequent = &'static str; // can be &str in no_std; use alloc::string::String if needed

#[cfg(test)]
#[mry::mry]
fn src_provider<'a>() -> Result<&'a str, &'static str> {
  Ok(DATA)
}

#[cfg(not(test))]
fn src_provider<'a>() -> Result<&'a str, &'static str> {
  Ok(DATA)
}

pub mod prelude {
  use super::*;

  pub fn extract<'a>() -> Result<ProblemDefinition<'a>, &'static str> {
    Ok(ProblemDefinition { input: src_provider()? })
  }

  pub fn transform<'a>(
    _data: ProblemDefinition<'a>,
  ) -> Result<Consequent, &'static str> {
    // you implement this per-day
    Err("unimplemented")
  }

  pub fn load(
    _result: Result<Consequent, &'static str>,
  ) -> Result<(), &'static str> {
    // also implemented per-day
    Err("unimplemented")
  }
}

#[cfg(test)]
mod tests {
  // normal tests work because cargo test enables std by default
}
