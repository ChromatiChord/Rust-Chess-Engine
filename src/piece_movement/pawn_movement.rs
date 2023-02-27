use crate::config::Player::White;
use crate::config::Player::Black;

pub fn get_pawn_moves(square: (i8, i8), 
occupied_self: Vec<(i8, i8)>, 
occupied_enemy: Vec<(i8, i8)>, 
enpassant_sqaure: (i8, i8), 
active_player: Player) -> Vec<(i8, i8)> {
    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    
    let double_rank = if active_player == White {2} else {7};
    // check which direction the pawn is meant to be moving
    let direction = if active_player == White {1} else {-1};
       
    // check if pawn can jump square ahead
    let new_square = (square.0 + direction, square.1);
    let mut one_ahead_occupied = true;
    if !occupied_self.contains(new_square) && !occupied_enemy.contains(new_square) {
        possible_squares.push(new_square);
        one_ahead_occupied = false;
    } 
    
    // check if we can jump 2 spaces
    if square.0 == double_rank {
        let new_square = (square.0 + direction * 2, square.1);
        if !occupied_self.contains(new_square) && !occupied_enemy.contains(new_square) {
            if !one_ahead_occupied {
                possible_squares.push(new_square);
            }
        }
    }

    // en passant check
    if occupied_enemy.contains((square.0 + direction, square.1 + 1)) {
        possible_squares.push((square.0 + direction, square.1 + 1));
    }
    if occupied_enemy.contains((square.0 + direction, square.1 - 1)) {
        possible_squares.push((square.0 + direction, square.1 - 1));
    }
    
    possible_squares
}