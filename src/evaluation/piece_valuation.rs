use crate::config;
use config::Piece;

const KING_VALUE: i8 = 0;
const QUEEN_VALUE: i8 = 9;
const BISHOP_VALUE: i8 = 3;
const KNIGHT_VALUE: i8 = 3;
const ROOK_VALUE: i8 = 5;
const PAWN_VALUE: i8 = 1;

pub fn get_piece_value(piece: Piece, square: (i8, i8)) -> i8 {
    match piece {
        Piece::Knight => KNIGHT_VALUE,
        Piece::Rook => ROOK_VALUE,
        Piece::Bishop => BISHOP_VALUE,
        Piece::King => KING_VALUE,
        Piece::Queen => QUEEN_VALUE,
        Piece::Pawn => PAWN_VALUE,
    }
}