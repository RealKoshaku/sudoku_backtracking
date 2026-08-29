mod sudoku_grid;
use sudoku_grid::Sudoku;

fn main() {
    let sudoku = Sudoku::new([[10; 9]; 9]);
    match sudoku.is_number_valid() {
        true => println!("Is valid !"),
        false => println!("Is not valid !"),
    }
}
