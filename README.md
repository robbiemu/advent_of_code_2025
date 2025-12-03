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
