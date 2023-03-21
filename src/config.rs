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
pub struct BoardState {
	pub white_pieces: Vec<PieceInfo>,
	pub black_pieces: Vec<PieceInfo>,
	pub occupied_white: Vec<(i8, i8)>,
	pub occupied_black: Vec<(i8, i8)>,
	pub active_player: Player,
	pub castle_rights: CastleRights,
	pub enpassant_square: Option<(i8, i8)>,
}

#[derive(Debug)]
pub enum SpecialAction {
    Promote,
    Castle,
    Enpassant
}

#[derive(Debug)]
pub struct PieceMovement {
    pub new_square: (i8, i8),
    pub special_action: Option<SpecialAction>
}

#[derive(Debug)]
pub struct CastleRights {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}
