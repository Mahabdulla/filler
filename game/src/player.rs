// Player state and identification
// Tracks player number, symbols, and game statistics

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerRole {
	P1,
	P2,
}

#[derive(Debug, Clone)]
pub struct Player {
	pub role: PlayerRole,
}

impl Player {
	pub fn new(role: PlayerRole) -> Self {
		Self { role }
	}

	/// Returns (my_last_char, my_territory_char, enemy_last_char, enemy_territory_char)
	/// For P1: ('a', '@', 's', '$')
	/// For P2: ('s', '$', 'a', '@')
	pub fn get_symbols(&self) -> (char, char, char, char) {
		match self.role {
			PlayerRole::P1 => ('a', '@', 's', '$'),
			PlayerRole::P2 => ('s', '$', 'a', '@'),
		}
	}

	/// Parse the first engine line like: "$$$ exec p1 : [robots/bender]"
	/// Returns Some(Player) if it contains p1 or p2, otherwise None.
	pub fn from_exec_line(line: &str) -> Option<Self> {
		if line.contains("p1") {
			Some(Player::new(PlayerRole::P1))
		} else if line.contains("p2") {
			Some(Player::new(PlayerRole::P2))
		} else {
			None
		}
	}
}