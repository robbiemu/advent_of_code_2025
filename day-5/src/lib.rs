#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod prelude {
  #[cfg(not(feature = "part2"))]
  pub mod part1_solver {
    include!("./part1.rs");
  }
  #[cfg(not(feature = "part2"))]
  pub use part1_solver::part1;

  #[cfg(feature = "part2")]
  pub mod part2_solver {
    include!("./part2.rs");
  }
  #[cfg(feature = "part2")]
  pub use part2_solver::part2;

  #[cfg(not(feature = "part2"))]
  pub mod ranged_search {
    include!("./ranged_search.rs");
  }
  #[cfg(not(feature = "part2"))]
  pub use ranged_search::RangeSearch;

  pub mod merge {
    include!("./merge.rs");
  }
  pub use merge::merge_intervals;
}
