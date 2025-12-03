# advent_of_code_2025
_see: [Advent of Code](http://adventofcode.com) website_

<p align="center">
  <img width="66%" alt="advent-of-code image for 2025, featuring a split design to reflect the halving of the number of puzzles" src="https://github.com/user-attachments/assets/6474eb98-59f1-4265-a1d1-8050d6cfa22d" />
</p>

This year I'm using Advent of Code to practice Rust with `no_std` —
at least while it’s still easy. :)

---

## day 1

Got my template working for `no_std`.

Benchmarks:

```
╰─ bench_part1  52.08 µs      │ 80.62 µs      │ 52.24 µs      │ 53.2 µs       │ 100     │ 100
╰─ bench_part2  55.2 µs       │ 87.66 µs      │ 55.41 µs      │ 56.95 µs      │ 100     │ 100
```

`no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 10,592 bytes (`day-1/target/lib-part1/release/libday_1.rlib`)
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 10,208 bytes (`day-1/target/lib-part2/release/libday_1.rlib`)

---

## day 2

nicely:

* No heap allocation
* No `Vec`, `HashMap`, `String`, formatting
* Boundary-safe slice manipulation
* Borrowed data only
* Feature-gated logic
* Separation of Part 1 / Part 2 behavior
* No unnecessary `unsafe`

Benchmarks:

```
╰─ bench_part1  17.55 ms      │ 19.48 ms      │ 18.42 ms      │ 18.45 ms      │ 100     │ 100
╰─ bench_part2  38.15 ms      │ 40.77 ms      │ 39.38 ms      │ 39.42 ms      │ 100     │ 100
```

Note: These times are slower than I’d like because the solver literally walks every integer in each range, converts it to decimal, reverses that buffer, and only then checks the repeated-pattern rule. Most of that work gets thrown away—millions of numbers never match—even though a more direct “generate the mirrored numbers and clamp to the range” strategy would skip the heavy per-value cost.

`no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 21,416 bytes (`day-2/target/lib-part1/release/libday_2.rlib`)
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 21,392 bytes (`day-2/target/lib-part2/release/libday_2.rlib`)

---

## day 3

first attempt at specifically targeting embedded systems:

* **No division or modulo** anywhere (important on MCUs where these are software-emulated)
* **No dynamic integer scaling** — all powers of 10 come from a compile-time constant table
* **Const-evaluated `POW10` array**, built entirely in `const` context
* **Fixed-size DP with no recomputation**: updates are (O(1)) per digit with predictable cost
* **Two-buffer DP update** to preserve correct subsequence ordering without digit reuse
* **Zero dynamic memory, zero formatting** — purely numeric hot loop
* **Single-pass algorithm**; no rescanning, no backtracking, no searching
* **Stable, branch-minimal inner loop** (only comparisons and additions/mults)
* **No `unsafe`**, and no architecture-specific assumptions
* **Deterministic timing and memory footprint**, suitable for constrained MCUs

Benchmarks:

```
╰─ bench_part1  17.87 µs      │ 23.24 µs      │ 18.29 µs      │ 18.49 µs      │ 100     │ 100
╰─ bench_part2  193.4 µs      │ 224.9 µs      │ 198.9 µs      │ 200.9 µs      │ 100     │ 100
```

`no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 5,632 bytes (`day-3/target/lib-part1/release/libday_3.rlib`)
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 9,752 bytes (`day-3/target/lib-part2/release/libday_3.rlib`)
