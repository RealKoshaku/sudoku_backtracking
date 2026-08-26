#[derive(Debug)]
///Represents a sudoku grid.
pub struct Sudoku {
    /// Represents the grid
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    /// Create a new `Sudoku` with a specified grid
    ///
    /// # Examples
    ///
    /// ```
    /// use sudokuGrid::Sudoku;
    ///
    /// let sudoku = Sudoku::new([[0; 9]; 9]);
    /// ```
    /// This create a sudoku grid with zeroes.
    ///
    /// ```
    /// use sudokuGrid::Sudoku;
    ///
    /// let a: [[u8; 9]; 9] = [
    /// [5, 3, 9, 8, 7, 6, 4, 1, 2],
    /// [7, 2, 8, 3, 1, 4, 9, 6, 5],
    /// [6, 4, 1, 2, 9, 5, 7, 3, 8],
    /// [4, 6, 2, 5, 3, 9, 8, 7, 1],
    /// [3, 8, 5, 7, 2, 1, 6, 4, 9],
    /// [1, 9, 7, 4, 6, 8, 2, 5, 3],
    /// [2, 5, 6, 1, 8, 7, 3, 9, 4],
    /// [9, 1, 3, 6, 4, 2, 5, 8, 7],
    /// [8, 7, 4, 9, 5, 3, 1, 2, 6]
    /// ]
    /// let sudoku = Sudoku::new(a)
    ///
    /// ```
    pub fn new(grid: [[u8; 9]; 9]) -> Self {
        Self { grid }
    }

    /// Verify if all the numbers of a given grid are smaller than 9.
    pub fn is_valid(&self) -> bool {
        for sub_grid in self.grid {
            for n in sub_grid {
                if n > 9 {
                    return false;
                } else {
                    continue;
                }
            }
        }
        true
    }
}
