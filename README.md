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

### Benchmarks:

```
╰─ bench_part1  52.08 µs      │ 80.62 µs      │ 52.24 µs      │ 53.2 µs       │ 100     │ 100
╰─ bench_part2  55.2 µs       │ 87.66 µs      │ 55.41 µs      │ 56.95 µs      │ 100     │ 100
```

### `no_std` library builds:

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

### Benchmarks:

```
╰─ bench_part1  17.55 ms      │ 19.48 ms      │ 18.42 ms      │ 18.45 ms      │ 100     │ 100
╰─ bench_part2  38.15 ms      │ 40.77 ms      │ 39.38 ms      │ 39.42 ms      │ 100     │ 100
```

Note: These times are slower than I’d like because the solver literally walks every integer in each range, converts it to decimal, reverses that buffer, and only then checks the repeated-pattern rule. Most of that work gets thrown away—millions of numbers never match—even though a more direct “generate the mirrored numbers and clamp to the range” strategy would skip the heavy per-value cost.

### `no_std` library builds:

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

### Benchmarks:

```
╰─ bench_part1  17.87 µs      │ 23.24 µs      │ 18.29 µs      │ 18.49 µs      │ 100     │ 100
╰─ bench_part2  193.4 µs      │ 224.9 µs      │ 198.9 µs      │ 200.9 µs      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 5,632 bytes (`day-3/target/lib-part1/release/libday_3.rlib`)
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 9,752 bytes (`day-3/target/lib-part2/release/libday_3.rlib`)

---

## day 4

kept the solver `no_std` / `no_alloc`, even though Part 2 needs hefty scratch space:

* Callers hand in every byte of working memory (`present`, `degree`, queue ring buffer + membership bitmap) so the library never touches an allocator.
* Sliding three-row window builds neighbor degrees without copying the grid, letting the hot loop stay branch-light.
* `TinySetQueue` reuses caller-owned slices to deliver set-like queue semantics in constant space.
* Mirrors embedded device patterns — e.g. a USB device controller firmware chews through host-provided DMA buffers without ever allocating, just like this solver consumes externally allocated slices.

### Benchmarks:

```
╰─ bench_part1  39.24 µs      │ 94.41 µs      │ 39.35 µs      │ 41.05 µs      │ 100     │ 100
╰─ bench_part2  150.9 µs      │ 328.9 µs      │ 152.6 µs      │ 157.9 µs      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 8,416 bytes (`day-4/target/lib-part1/release/libday_4.rlib`)
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 29,144 bytes (`day-4/target/lib-part2/release/libday_4.rlib`)


## day 5

First steps toward a **`no_std`, HFT-flavored design**.

The puzzle itself is simple (count IDs that fall inside range boundaries), but I used it as an excuse to play with patterns you see in **high-performance trading systems**. I originally reached for binary search and needed dynamic storage, so once `alloc` was in the picture, I pivoted and leaned into the HFT vibe:

* **Structure-of-Arrays layout** (`starts[]` / `ends[]`) for cache-friendly access
* **SIMD membership checks** with `wide::u64x4`
* **Predictable hot loop** (4 intervals per iteration)
* **Branchless scalar tail** so timing stays uniform

It’s overkill for AoC input sizes, but great practice for writing code with **deterministic cost, cache coherence, and minimal wasted work**. (It only comes into play in part1 -- I had already decided to pre-merge then and that is the engine to part2)

### Benchmarks:

```
bench           fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_part1  38.83 µs      │ 87.45 µs      │ 39.22 µs      │ 42.89 µs      │ 100     │ 100
╰─ bench_part2  26.49 µs      │ 51.33 µs      │ 27.87 µs      │ 28.96 µs      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 43376 *size depends on SIMD choice*
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 17672 *same core, no additional memory needs*

## day 6

Being the weekend, I started very late and kinda struggled with the meticulous nature of transformations in part 2. As a result got my first "bronze" medal ie millisecond benchmarks.

### Benchmarks:

```
╰─ bench_part1  34.24 µs      │ 77.45 µs      │ 34.39 µs      │ 35.49 µs      │ 100     │ 100
╰─ bench_part2  1.215 ms      │ 2.33 ms       │ 1.452 ms      │ 1.447 ms      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 12,272 bytes
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 21,200 bytes

## day 7

Formalized the **Owner/View** memory architecture that I've been loosely using since Day 5.

Instead of creating ad-hoc vectors in `main` and passing loose slices into the solver, I created a dedicated `ProblemData` struct (in the `std` parsing module) to act as the memory **Owner**. The solver operates on a `Problem` struct (the **View**), which contains nothing but borrowed mutable slices.

For Part 2, the "Quantum" splitting implied exponential growth ($2^N$). I avoided heap explosions by treating the grid like a **streaming signal**:

*   **Formalized Owner/View:** `ProblemData` (std) handles allocation; `Problem` (no_std) handles logic.
*   **Double-Buffered Simulation:** Part 2 uses two fixed-width row buffers ("Current" and "Next") to track particle counts.
*   **Zero-Copy Swapping:** The simulation toggles between the two buffers (`core::mem::swap`) without moving data, processing the grid row-by-row.
*   **Hardware Analogy:** This mirrors **Line Buffering** in embedded video controllers or DMA transfers, where hardware processes one scanline while receiving the next, minimizing memory requirements to $O(Width)$ rather than $O(Total)$.

### Benchmarks:

```
bench           fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_part1  31.91 µs      │ 67.83 µs      │ 32.16 µs      │ 34.13 µs      │ 100     │ 100
╰─ bench_part2  34.79 µs      │ 55.2 µs       │ 35.66 µs      │ 36.58 µs      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 7,512 bytes
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 6,904 bytes

## day 8

Part 1 was a small union find: keep the **K shortest edges** and use a DSU to measure circuit sizes. Easy enough in `no_std`, but laborious for me because I did BFS before realizing that was the wrong choice.

Part 2 looked similar… until the graph turned out to be **complete**.
With 1000 points, that’s ~500k edges — far too many for a `heapless::BinaryHeap`, which stores its entire backing array _on the stack_. The result: instant stack overflow.

So I switched to Prim’s to trade in some complexity for less space:

* No edge storage
* O(N²) predictable work
* Only a handful of fixed-size arrays
* `no_std` friendly

It’s a nice reminder that algorithms you’d pick on a desktop aren’t always the ones you pick when you're pretending RAM is measured in kilobytes instead of gigabytes.

### Benchmarks:

```
bench           fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ bench_part1  615.9 µs      │ 1.013 ms      │ 661.9 µs      │ 681.7 µs      │ 100     │ 100
╰─ bench_part2  1.677 ms      │ 2.003 ms      │ 1.699 ms      │ 1.729 ms      │ 100     │ 100
```

### `no_std` library builds:

* Part 1: `cargo build --release --lib --target-dir target/lib-part1` → 28,664 bytes
* Part 2: `cargo build --release --lib --features part2 --target-dir target/lib-part2` → 24,544 bytes
