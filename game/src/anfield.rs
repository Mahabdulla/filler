// Anfield (game board) representation and logic
// Manages the grid state, player territories, and placement validation

#[derive(Debug, Clone)]
pub struct Anfield {
	pub width: usize,
	pub height: usize,
	// Grid indexed as grid[y][x]
	pub grid: Vec<Vec<char>>,
}

impl Anfield {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			grid: vec![vec!['.'; width]; height],
		}
	}

	/// Parse an Anfield from lines. Lines may include row numbers/prefixes (e.g. "000 ....").
	/// This function extracts the first run of board characters starting at the first
	/// occurrence of a board char ('.', 'a', 's', '@', '$').
	pub fn from_lines(lines: &[String]) -> Self {
		let mut rows: Vec<Vec<char>> = Vec::new();
		for line in lines {
			if line.trim().is_empty() {
				continue;
			}
			// find the index of the first board character
			let idx = line
				.find(|c: char| c == '.' || c == 'a' || c == 's' || c == '@' || c == '$')
				.unwrap_or(0);
			let row_str = &line[idx..];
			rows.push(row_str.chars().collect());
		}
		let height = rows.len();
		let width = if height > 0 { rows[0].len() } else { 0 };
		Self {
			width,
			height,
			grid: rows,
		}
	}

	pub fn in_bounds(&self, x: i32, y: i32) -> bool {
		x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
	}

	pub fn get(&self, x: i32, y: i32) -> Option<char> {
		if self.in_bounds(x, y) {
			Some(self.grid[y as usize][x as usize])
		} else {
			None
		}
	}

	pub fn set(&mut self, x: i32, y: i32, ch: char) {
		if self.in_bounds(x, y) {
			self.grid[y as usize][x as usize] = ch;
		}
	}

	/// Check if a piece can be placed at (x, y) for a given player.
	///
	/// Rules:
	/// 1. The piece must be within the bounds of the Anfield.
	/// 2. The piece must overlap exactly one cell of the player's territory.
	/// 3. The piece must not overlap any cell of the opponent's territory.
	pub fn can_place(
		&self,
		piece: &crate::piece::Piece,
		x: i32,
		y: i32,
		my_chars: &[char],
		enemy_chars: &[char],
	) -> bool {
		let mut overlap_count = 0;

		for &(px, py) in &piece.shape {
			let abs_x = x + px;
			let abs_y = y + py;

			// 1. Check bounds
			if !self.in_bounds(abs_x, abs_y) {
				return false;
			}

			let cell = self.get(abs_x, abs_y).unwrap();

			// 2. Check enemy collision
			if enemy_chars.contains(&cell) {
				return false;
			}

			// 3. Check overlap with own territory
			if my_chars.contains(&cell) {
				overlap_count += 1;
			}
		}

		// Must overlap exactly one of my cells
		overlap_count == 1
	}
}