mod sudokuGrid;
use sudokuGrid::Sudoku;

fn main() {
    let sudoku = Sudoku::new([[0; 9]; 9]);
    match sudoku.is_valid() {
        true => println!("Is valid !"),
        false => println!("Is not valid !"),
    }
}
