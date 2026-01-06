use crate::anfield::Anfield;
use crate::piece::Piece;

/// Simple but effective strategy: Place pieces as close as possible to enemy territory.
/// This is aggressive and consistent - exactly what beats most opponents.
pub fn solve_heatmap(
    anfield: &Anfield,
    piece: &Piece,
    my_chars: &[char],
    enemy_chars: &[char],
) -> (i32, i32) {
    // Step 1: Find all enemy positions
    let mut enemy_positions = Vec::new();
    for y in 0..anfield.height {
        for x in 0..anfield.width {
            if let Some(ch) = anfield.get(x as i32, y as i32) {
                if enemy_chars.contains(&ch) {
                    enemy_positions.push((x as i32, y as i32));
                }
            }
        }
    }

    // If no enemies, just return first valid position
    if enemy_positions.is_empty() {
        for y in 0..anfield.height as i32 {
            for x in 0..anfield.width as i32 {
                if anfield.can_place(piece, x, y, my_chars, enemy_chars) {
                    return (x, y);
                }
            }
        }
        return (0, 0);
    }

    // Step 2: Find the best valid position
    let mut best_distance = f64::INFINITY;
    let mut best_position = (0, 0);
    let mut found_valid = false;

    for y in 0..anfield.height as i32 {
        for x in 0..anfield.width as i32 {
            if anfield.can_place(piece, x, y, my_chars, enemy_chars) {
                // Calculate minimum distance to ANY enemy cell
                let mut min_distance_to_enemy = f64::INFINITY;
                
                for (enemy_x, enemy_y) in &enemy_positions {
                    // Use Euclidean distance (like your friend)
                    let dx = (x - enemy_x) as f64;
                    let dy = (y - enemy_y) as f64;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    if distance < min_distance_to_enemy {
                        min_distance_to_enemy = distance;
                    }
                }

                // If this position is closer to enemies, choose it
                if min_distance_to_enemy < best_distance {
                    best_distance = min_distance_to_enemy;
                    best_position = (x, y);
                    found_valid = true;
                }
            }
        }
    }

    if found_valid {
        best_position
    } else {
        (0, 0)
    }
}
