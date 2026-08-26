#[derive(Debug)]
struct sudoku {
	grid: [[u8; 9]; 9],
}

impl sudoku {
	fn new(grid: [u9; 9]; 9) -> Self {
		Self { grid }
	}
	
	fn is_valid(&self) {
		for sub_grid in self.grid {
			for n in sub_grid {
				if not (0 <= n && n <= 9) {
					false
				}
			}
		}
		true
	}
}