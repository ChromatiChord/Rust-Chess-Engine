use crate::config;
use config::Piece;
use config::Player;
use config::PieceInfo;
use config::CastleRights;

use crate::piece_movement;
use piece_movement::piece_movement_brains;

use crate::evaluation;
use evaluation::piece_valuation::get_piece_value;

pub fn fen_deconstruct(fen: &str) -> config::BoardState {
	// deconstruct FEN into major components
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	let constructed_board_info = construct_board(deconstructed_fen[0]);

	let boardstate = config::BoardState {
		white_pieces: constructed_board_info.0,
		black_pieces: constructed_board_info.1,
		occupied_white: constructed_board_info.2,
		occupied_black: constructed_board_info.3,
		active_player: if deconstructed_fen[1] == "w" {Player::White} else {Player::Black},
		castle_rights: analyse_castle_rights(deconstructed_fen[2]),
		enpassant_square: enpassant_deconstruct(deconstructed_fen[3]),
	};
	// println!("{:?}", boardstate);
	
    boardstate
}

// takes in the pieces from FEN and gives back a board state
// for use in the final boardstate object
pub fn construct_board(pieces: &str) -> ( Vec<PieceInfo>, Vec<PieceInfo>, Vec<(i8, i8)>, Vec<(i8, i8)> ) {
	// splits up the pieces into their own ranks
	let ranks: Vec<&str> = pieces.split("/").collect();
	// generates empty final board
	let mut white_pieces = Vec::new();
	let mut black_pieces = Vec::new();

	let mut occupied_white = Vec::new();
	let mut occupied_black = Vec::new();
	
	// loop through ranks
	for (rank_iter, rank) in ranks.iter().enumerate() {
		// loop through files
		for (file_iter, file) in rank.chars().enumerate() {
			let mut piece_at_pos_result = get_piece_at_pos(file, rank_iter as i8, file_iter as i8);

			match piece_at_pos_result {
				Some(new_piece) => {
					match new_piece.owner {
						Player::White => { 
							white_pieces.push(new_piece.clone());
							occupied_white.push(new_piece.square);
						}
						Player::Black => {
							black_pieces.push(new_piece.clone());
							occupied_black.push(new_piece.square.clone());
						}
					};
				},
				None => ()
			}

		}
	}
	
	( white_pieces, black_pieces, occupied_white, occupied_black )
}

// required for parsing values such as '5' (5 empty spaces in a row)
// if a piece is detected, return details just returns the piece value itself
fn get_piece_at_pos(piece: char, rank_iter: i8, file_iter: i8) -> Option<PieceInfo> {
	
	if piece.is_numeric() {
		return None;
	}
	

	let piece_owner = if piece.is_uppercase() {Player::White} else {Player::Black};

	let piece_type_lowered = piece.to_lowercase().next().unwrap();
	// println!("{:?}", piece_type_lowered);

	let piece_type = match piece_type_lowered {
		'k' => Piece::King,
		'q' => Piece::Queen,
		'r' => Piece::Rook,
		'n' => Piece::Knight,
		'b' => Piece::Bishop,
		'p' => Piece::Pawn,
		 _  => panic!("Inputted piece is not a real piece! Piece: {:?}", piece_type_lowered)
	};

	let piece_square = (rank_iter, file_iter);

	let mut piece_info = PieceInfo {
		piece_type: piece_type.clone(),
		square: piece_square,
		piece_value: get_piece_value(piece_type.clone(), piece_square),
		owner: piece_owner
	};
	
	return Some(piece_info);
}


fn analyse_castle_rights(rights: &str) -> CastleRights {
	let decomp_rights: Vec<char> = rights.to_string().chars().collect();
	let mut return_rights = CastleRights {
		white_short: false,
		white_long: false,
		black_short: false,
		black_long: false,
	};

	for right in decomp_rights {
		// white player rights
		if right.is_uppercase() {
			if right == 'K' {
				return_rights.white_short = true;
			}
			if right == 'Q' {
				return_rights.white_long = true;
			}
		} 
		// black player rights
		else {
			if right == 'k' {
				return_rights.black_short = true;
			}
			if right == 'q' {
				return_rights.black_long = true;
			}
		}
	}
	return_rights
}

// takes a enpasssant square string such as "d4",
// and converts to coords: (3, 3)
fn enpassant_deconstruct(square: &str) -> (i8, i8) {
	let squ_deconstruct: Vec<char> = square.to_string().chars().collect();
	if squ_deconstruct.len() == 1 {
		return (10, 10);
	}
	
	(squ_deconstruct[0] as i8 - 97, squ_deconstruct[1] as i8 - '0' as i8 - 1)

}