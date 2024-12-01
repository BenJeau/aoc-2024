# Advent of Code 2024

This repository contains my solutions in Rust to the [Advent of Code 2024](https://adventofcode.com/2024) challenges.

## Usage

To run the solutions, you need to have [Rust](https://www.rust-lang.org/) installed. Then you can run the solutions with:

```bash
cargo run --bin day1
```

There's a Rust binary created for every day with the inputs of the questions in the `input` folder.

## Development of solutions

When developing the solutions, having the following running is useful to automatically run it against the real input after file changes (this requires https://crates.io/crates/cargo-watch):

```bash
cargo watch -x 'run --bin day1 --release'
```
