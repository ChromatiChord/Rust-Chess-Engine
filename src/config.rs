#[derive(Debug)]
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

#[derive(Debug)]
pub struct BoardState<'board> {
	pub board_state: Vec<Vec<char>>,
	pub occupied_white: Vec<(i8, i8)>,
	pub occupied_black: Vec<(i8, i8)>,
	pub active_player: Player,
	pub castle_rights: &'board str,
	pub enpassant_square: &'board str,
}
