use crate::config;

pub fn fen_deconstruct(fen: &str) -> config::BoardState {
	// deconstruct FEN into major components
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	
	let boardstate = config::BoardState {
		board_state: construct_board(deconstructed_fen[0]),
		active_player: if deconstructed_fen[1] == "w" {config::Player::White} else {config::Player::Black},
		castle_rights: deconstructed_fen[2],
		enpassant_square: deconstructed_fen[3],
	};
	// println!("{:?}", boardstate);
    
    boardstate
}

// takes in the pieces from FEN and gives back a board state
// for use in the final boardstate object
pub fn construct_board(pieces: &str) -> Vec<Vec<char>> {
	// splits up the pieces into their own ranks
	let ranks: Vec<&str> = pieces.split("/").collect();
	// generates empty final board
	let mut final_board = Vec::new();
	
	// loop through ranks
	for rank in ranks {
		let mut new_rank = Vec::new();
		// loop through files
		for file in rank.chars() {
			let mut new_file = get_new_file(file);
			new_rank.append(&mut new_file);
		}
		final_board.push(new_rank);
	}
	
	final_board
}

fn get_new_file(file: char) -> Vec<char> {
	let mut new_file = Vec::new();

	if file.is_numeric() {
		let empty_square_num: i32 = file.to_string().parse().unwrap();
		for _ in 0..empty_square_num {
			new_file.push('_');
		}
	} else {
		new_file.push(file);
	}

	new_file
}