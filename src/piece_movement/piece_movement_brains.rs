use crate::config::Player::White;
use crate::config::Player::Black;

use super::bishop_movement::get_bishop_moves;
use super::king_movement::get_king_moves;

pub fn get_available_moves(piece: char, occupied_white: Vec<(i32, i32)>, occupied_black: Vec<(i32, i32)>) {
    let active_player = if piece.is_uppercase() {White} else {Black};
    let converted_piece = piece.to_ascii_uppercase();

    // let available_squares: Vec<(i8, i8)> =
    match converted_piece {
        'B' => get_bishop_moves(occupied_white, occupied_black),
        _ => get_king_moves(occupied_white, occupied_black)
    }
}