#[derive(Debug, Clone, Copy)]
pub enum Player {
	White,
	Black
}

pub fn switch_player(player: Player) -> Player {
    match player {
        Player::White => return Player::Black,
        Player::Black => return Player::White
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Knight,
    Bishop,
    Queen,
    King,
    Rook,
    Pawn,
}

#[derive(Debug, Clone)]
pub struct PieceInfo {
    pub piece_type: Piece,
    pub square: (i8, i8),
    pub piece_value: i8,
    pub owner: Player
}

#[derive(Debug)]
pub struct BoardState<'board> {
	pub white_pieces: Vec<PieceInfo>,
	pub black_pieces: Vec<PieceInfo>,
	pub occupied_white: Vec<(i8, i8)>,
	pub occupied_black: Vec<(i8, i8)>,
	pub active_player: Player,
	pub castle_rights: &'board str,
	pub enpassant_square: &'board str,
}


