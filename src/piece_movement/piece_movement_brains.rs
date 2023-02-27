use crate::config::Player::White;
use crate::config::Player::Black;

use super::rook_movement::get_rook_moves;
use super::knight_movement::get_king_moves;
use super::bishop_movement::get_bishop_moves;
use super::queen_movement::get_queen_moves;
use super::king_movement::get_king_moves;
use super::pawn_movement::get_pawn_moves;

pub fn get_available_moves(piece: char, coords: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) ->
    Vec<(i8, i8)> {
    let active_player = if piece.is_uppercase() {White} else {Black};
    let converted_piece = piece.to_ascii_uppercase();

    let available_squares: Vec<(i8, i8)> = match converted_piece {
        'R' => get_rook_moves(coords, occupied_self, occupied_enemy),
        'N' => get_knight_moves(coords, occupied_self, occupied_enemy),
        'B' => get_bishop_moves(coords, occupied_self, occupied_enemy),
        'Q' => get_queen_moves(coords, occupied_self, occupied_enemy),
        'K' => get_king_moves(coords, occupied_self, occupied_enemy),
        'P' => get_pawn_moves(coords, occupied_self, occupied_enemy),
        _ => panic!("Inputted piece is not a real piece!")
    };
    available_squares
}