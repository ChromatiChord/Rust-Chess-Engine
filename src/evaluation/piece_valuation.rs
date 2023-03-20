use crate::config;
use config::Piece;

const king_value: i8 = 0;
const queen_value: i8 = 9;
const bishop_value: i8 = 3;
const knight_value: i8 = 3;
const rook_value: i8 = 5;
const pawn_value: i8 = 1;

pub fn get_piece_value(piece: Piece, square: (i8, i8)) -> i8 {
    match piece {
        Piece::Knight => knight_value,
        Piece::Rook => rook_value,
        Piece::Bishop => bishop_value,
        Piece::King => king_value,
        Piece::Queen => queen_value,
        Piece::Pawn => pawn_value,
    }
}