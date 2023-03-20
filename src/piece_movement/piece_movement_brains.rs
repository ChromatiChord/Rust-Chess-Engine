use crate::config;
use config::Player;
use config::Player::White;
use config::Player::Black;
use config::Piece;

use super::rook_movement::get_rook_moves;
use super::knight_movement::get_knight_moves;
use super::bishop_movement::get_bishop_moves;
use super::queen_movement::get_queen_moves;
use super::king_movement::get_king_moves;
use super::pawn_movement::get_pawn_moves;

pub fn get_available_moves(
    piece_type: &Piece, 
    active_player: &Player,
    coords: &(i8, i8), 
    occupied_white: &Vec<(i8, i8)>, 
    occupied_black: &Vec<(i8, i8)>, 
    enpassant_square: (i8, i8)) -> Vec<(i8, i8)> {
        let occupied_self = match active_player {
            White => occupied_white.clone(),
            Black => occupied_black.clone()
        };
        let occupied_enemy = match active_player {
            White => occupied_black.clone(),
            Black => occupied_white.clone()
        };
        
        let available_squares: Vec<(i8, i8)> = match piece_type {
            Piece::Rook => get_rook_moves(*coords, occupied_self, occupied_enemy),
            Piece::Knight => get_knight_moves(*coords, occupied_self, occupied_enemy),
            Piece::Bishop => get_bishop_moves(*coords, occupied_self, occupied_enemy),
            Piece::Queen => get_queen_moves(*coords, occupied_self, occupied_enemy),
            Piece::King => get_king_moves(*coords, occupied_self, occupied_enemy),
            Piece::Pawn => get_pawn_moves(*coords, occupied_self, occupied_enemy, enpassant_square, *active_player),
            _ => panic!("Inputted piece is not a real piece!")
        };
        available_squares
}