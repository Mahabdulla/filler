// Entry point for the Filler game bot
// Handles I/O with the game engine: reads input from stdin, processes game state, and outputs piece placement coordinates

mod anfield;
mod piece;
mod player;
mod strategy;

use std::io::{self, BufRead};
use anfield::Anfield;
use piece::Piece;
use player::Player;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut player: Option<Player> = None;
    let mut anfield: Option<Anfield> = None;

    while let Some(Ok(line)) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("$$$ exec") {
            // Parse player info: "$$$ exec p1 : [robots/bender]"
            player = Player::from_exec_line(line);
        } else if line.starts_with("Anfield") {
            // Parse Anfield dimensions: "Anfield 20 15:"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let height: usize = parts[2].trim_matches(':').parse().unwrap_or(0);
                
                // Skip the header line (column indices)
                let _ = lines.next(); 

                // Read height lines
                let mut board_lines = Vec::new();
                for _ in 0..height {
                    if let Some(Ok(board_line)) = lines.next() {
                        board_lines.push(board_line);
                    }
                }
                anfield = Some(Anfield::from_lines(&board_lines));
            }
        } else if line.starts_with("Piece") {
            // Parse Piece dimensions: "Piece 4 1:"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let height: usize = parts[2].trim_matches(':').parse().unwrap_or(0);
                let width: usize = parts[1].parse().unwrap_or(0);
                
                // Read height lines for the piece
                let mut piece_lines = Vec::new();
                for _ in 0..height {
                    if let Some(Ok(piece_line)) = lines.next() {
                        piece_lines.push(piece_line);
                    }
                }
                let piece = Piece::from_lines(width, height, &piece_lines);

                // Make a move
                if let (Some(p), Some(a)) = (&player, &anfield) {
                    let (my_char, my_terr, enemy_char, enemy_terr) = p.get_symbols();
                    let my_chars = [my_char, my_terr];
                    let enemy_chars = [enemy_char, enemy_terr];
                    
                    let (x, y) = strategy::solve_heatmap(a, &piece, &my_chars, &enemy_chars);
                    println!("{} {}", x, y);
                }
            }
        }
    }
}

