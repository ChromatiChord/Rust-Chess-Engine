use crate::config::{self, SpecialAction, Player, Piece, CastleRights, BoardState, AvailablePieceMove, PieceInfo};

use super::rook_movement::get_rook_moves;
use super::knight_movement::get_knight_moves;
use super::bishop_movement::get_bishop_moves;
use super::queen_movement::get_queen_moves;
use super::king_movement::get_king_moves;
use super::pawn_movement::get_pawn_moves;


pub fn get_available_moves_from_state(board_state: &BoardState) -> Vec<AvailablePieceMove> {
    let mut available_moves: Vec<AvailablePieceMove> = vec![];
    match board_state.active_player {
        Player::White => {
            for piece in &board_state.white_pieces {
                available_moves.append(&mut get_available_moves(&piece, &board_state));
            }
        },
        Player::Black => {
            for piece in &board_state.black_pieces {
                available_moves.append(&mut get_available_moves(&piece, &board_state));
            }
        }
    }
    available_moves
}

fn get_available_moves(
    piece: &PieceInfo,
    board_state: &BoardState) -> Vec<AvailablePieceMove> {
        let occupied_self = match board_state.active_player {
            Player::White => board_state.occupied_white.clone(),
            Player::Black => board_state.occupied_black.clone()
        };
        let occupied_enemy = match board_state.active_player {
            Player::White => board_state.occupied_black.clone(),
            Player::Black => board_state.occupied_white.clone()
        };
        
        let mut available_moves = match piece.piece_type {
            Piece::Rook => get_rook_moves(piece, occupied_self, occupied_enemy),
            Piece::Knight => get_knight_moves(piece, occupied_self, occupied_enemy),
            Piece::Bishop => get_bishop_moves(piece, occupied_self, occupied_enemy),
            Piece::Queen => get_queen_moves(piece, occupied_self, occupied_enemy),
            Piece::King => get_king_moves(piece, occupied_self, occupied_enemy, board_state.castle_rights, board_state.active_player),
            Piece::Pawn => get_pawn_moves(piece, occupied_self, occupied_enemy, board_state.enpassant_square, board_state.active_player),
            _ => panic!("Inputted piece is not a real piece!")
        };
        available_moves
}