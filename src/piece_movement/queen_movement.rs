use super::bishop_movement::get_bishop_moves;
use super::rook_movement::get_rook_moves;

use crate::config::PieceMovementTrigger;

pub fn get_queen_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) ->
(Vec<(i8, i8)>, Vec<PieceMovementTrigger>) {
        let mut possible_squares: Vec<(i8, i8)> = Vec::new();

        possible_squares.append(&mut get_bishop_moves(square, occupied_self.clone(), occupied_enemy.clone()).0);
        possible_squares.append(&mut get_rook_moves(square, occupied_self, occupied_enemy).0);
        
        (possible_squares, vec![])
}