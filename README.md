# sudoku_backtracking

A Rust project implementing a Sudoku solver using a backtracking algorithm.

## Status

Work in progress. The project currently provides:

- [`Sudoku`](src/sudoku_grid.rs) — a type representing a 9x9 sudoku grid.
- [`Sudoku::new`](src/sudoku_grid.rs) — construct a grid from raw values.
- [`Sudoku::is_number_valid`](src/sudoku_grid.rs) — check that every cell contains a value in the valid range `0..=9`.

The backtracking solver itself is not implemented yet.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (edition 2024)

### Build

```sh
cargo build
```

### Run

```sh
cargo run
```

The binary builds a sample grid and prints whether it is valid.

### Test

```sh
cargo test
```

## Project Structure

```
src/
├── main.rs          # Program entry point
└── sudoku_grid.rs   # The Sudoku type and its validation methods
```

## License

See [LICENSE](LICENSE).
