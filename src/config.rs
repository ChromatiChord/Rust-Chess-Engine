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
	pub active_player: Player,
	pub castle_rights: &'board str,
	pub enpassant_square: &'board str,
}














// // (RANK, FILE)
// // (Y, X)

// pub let rook_movement: Vec<(i32, i32)> = vec![
//     //starts from up, clockwise
//     (-1, 0),
//     (0, 1),
//     (1, 0),
//     (0, -1)
// ];

// pub let bishop_movement: Vec<(i32, i32)> = vec![
//     (-1, 1),
//     (1, 1),
//     (1, -1),
//     (-1, -1)
// ];

// pub let royal_movement: Vec<(i32, i32)> = vec![
//     (-1, 1),
//     (1, 1),
//     (1, -1),
//     (-1, -1),
//     (-1, 0),
//     (0, 1),
//     (1, 0),
//     (0, -1)
// ];

// pub let knight_movement: Vec<(i32, i32)> = vec![
//     (-2, -1),
//     (-2, 1),
//     (-1, 2),
//     (1, 2),
//     (2, -1),
//     (2, 1),
//     (-1, -2),
//     (1, -2)
// ];
