// Piece structure and parsing
// Handles piece representation, parsing from game engine input, and piece transformations

#[derive(Debug, Clone)]
pub struct Piece {
	pub width: usize,
	pub height: usize,
	// Coordinates of filled cells relative to the piece top-left (x, y)
	pub shape: Vec<(i32, i32)>,
}

impl Piece {
	pub fn new(width: usize, height: usize, shape: Vec<(i32, i32)>) -> Self {
		Self {
			width,
			height,
			shape,
		}
	}

	/// Parse a piece from its textual representation lines.
	/// Each line should be a string of '.' for empty and any other char for a filled cell.
	pub fn from_lines(width: usize, height: usize, lines: &[String]) -> Self {
		let mut shape = Vec::new();
		for (y, line) in lines.iter().enumerate().take(height) {
			for (x, ch) in line.chars().enumerate().take(width) {
				if ch != '.' {
					shape.push((x as i32, y as i32));
				}
			}
		}
		Piece::new(width, height, shape)
	}

	/// Return absolute positions when the piece is placed with its top-left at (ox, oy)
	pub fn positions_at(&self, ox: i32, oy: i32) -> Vec<(i32, i32)> {
		self.shape.iter().map(|(x, y)| (ox + x, oy + y)).collect()
	}
}