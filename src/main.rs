// This module contains the core `Sudoku` type that represents and validates
// a sudoku grid. The crate is a work-in-progress; the backtracking solver
// will be implemented here in a future step.
pub mod sudoku_grid;

use sudoku_grid::Sudoku;

/// Entry point of the program.
///
/// Creates a sample `Sudoku` grid filled with an out-of-range value (`10`)
/// and prints whether the grid is valid according to the
/// [`Sudoku::is_number_valid`] check.
fn main() {
    // Build a grid where every cell contains the value 10, which is outside
    // the allowed sudoku range (0..=9).
    let sudoku = Sudoku::new([[10; 9]; 9]);

    // Display the validation result to the user.
    match sudoku.is_number_valid() {
        true => println!("Is valid !"),
        false => println!("Is not valid !"),
    }
}
