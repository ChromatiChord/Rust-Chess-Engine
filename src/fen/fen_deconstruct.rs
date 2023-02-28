use crate::config;

pub fn fen_deconstruct(fen: &str) -> config::BoardState {
	// deconstruct FEN into major components
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	let constructed_board_info = construct_board(deconstructed_fen[0]);

	let boardstate = config::BoardState {
		board_state: constructed_board_info.0,
		occupied_white: constructed_board_info.1,
		occupied_black: constructed_board_info.2,
		active_player: if deconstructed_fen[1] == "w" {config::Player::White} else {config::Player::Black},
		castle_rights: deconstructed_fen[2],
		enpassant_square: deconstructed_fen[3],
	};
	// println!("{:?}", boardstate);
	println!("asdsa");
	println!("{:?}", boardstate);
	println!("asds");
	
    boardstate
}

// takes in the pieces from FEN and gives back a board state
// for use in the final boardstate object
pub fn construct_board(pieces: &str) -> ( Vec<Vec<char>>, Vec<(i8, i8)>, Vec<(i8, i8)> ) {
	// splits up the pieces into their own ranks
	let ranks: Vec<&str> = pieces.split("/").collect();
	// generates empty final board
	let mut final_board = Vec::new();
	let mut occupied_white = Vec::new();
	let mut occupied_black = Vec::new();
	
	// loop through ranks
	for (rank_iter, rank) in ranks.iter().enumerate() {
		let mut new_rank = Vec::new();
		// loop through files
		for (file_iter, file) in rank.chars().enumerate() {
			let mut new_file = get_new_file(file, rank_iter as i8, file_iter as i8);
			match new_file.1 {
				Some(x) => if new_file.0[0].is_uppercase() {occupied_white.push(x)} else {occupied_black.push(x)},
				None => ()
			}
			new_rank.append(&mut new_file.0);
		}
		final_board.push(new_rank);
	}
	
	( final_board, occupied_white, occupied_black )
}

// required for parsing values such as '5' (5 empty spaces in a row)
// otherwise just returns the piece value itself
fn get_new_file(file: char, rank_iter: i8, file_iter: i8) -> ( Vec<char>, Option<(i8, i8)> ) {
	let mut new_file = Vec::new();

	if file.is_numeric() {
		let empty_square_num: i32 = file.to_string().parse().unwrap();
		for _ in 0..empty_square_num {
			new_file.push('_');
		}
		return (new_file, None);
	} else {
		new_file.push(file);
		return (new_file, Some((rank_iter, file_iter)));
	}
}