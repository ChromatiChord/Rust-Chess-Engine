#[derive(Debug, Clone, Copy)]
pub enum Player {
	White,
	Black
}

#[derive(Debug, Clone, Copy)]
pub enum Agent {
	Max,
	Min
}

pub fn switch_player(player: Player) -> Player {
    match player {
        Player::White => return Player::Black,
        Player::Black => return Player::White
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Knight,
    Bishop,
    Queen,
    King,
    Rook,
    Pawn,
}

// ------------ BIG BOY BOARD STATE ------------
#[derive(Debug, Clone)]
pub struct BoardState {
    pub white_pieces: Vec<PieceInfo>,
    pub black_pieces: Vec<PieceInfo>,
    pub occupied_white: Vec<(i8, i8)>,
    pub occupied_black: Vec<(i8, i8)>,
    pub active_player: Player,
    pub castle_rights: CastleRights,
    pub enpassant_square: Option<(i8, i8)>,
}

// stored in the board_state, where all a piece's data is
#[derive(Debug, Clone, Copy)]
pub struct PieceInfo {
    pub piece_type: Piece,
    pub square: (i8, i8),
    pub piece_value: i8,
    pub owner: Player
}

#[derive(Debug, Clone, Copy)]
pub enum SpecialAction {
    Promote,
    EnpassantGenerate,
    EnpassantAttack,
    CastleShort,
    CastleLong,
    DisableCastleShort,
    DisableCastleLong,
    Capture
}

// when generating moves, this is the structure we use to store what we iterate through
#[derive(Debug, Clone)]
pub struct AvailablePieceMove {
    pub piece: PieceInfo,
    pub new_square: (i8, i8),
    pub special_action: Option<Vec<SpecialAction>>
}

#[derive(Debug, Clone, Copy)]
pub struct CastleRights {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}
