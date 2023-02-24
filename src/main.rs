// use text_io::read;

#[derive(Debug)]
enum Player {
	White,
	Black
}

#[derive(Debug)]
struct BoardState<'board> {
	board_state: Vec<Vec<char>>,
	active_player: Player,
	castle_rights: &'board str,
	enpassant_square: &'board str,
}

// takes in the pieces from FEN and gives back a board state
// for use in the final boardstate object
fn construct_board(pieces: &str) -> Vec<Vec<char>> {
	// splits up the pieces into their own ranks
	let ranks: Vec<&str> = pieces.split("/").collect();
	// generates empty final board
	let mut final_board = Vec::new();
	
	// loop through ranks
	for (index, rank) in ranks.iter().enumerate() {
		let mut new_rank = Vec::new();
		// loop through files
		for file in rank.chars() {
			let mut new_file = get_new_file(file);
			new_rank.append(&mut new_file);
		}
		final_board.push(new_rank);
	}
	
	return final_board;
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

	return new_file
}

fn deconstruct_fen(fen: &str) {
	// deconstruct FEN into major components
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	
	let mut boardstate = BoardState {
		board_state: construct_board(deconstructed_fen[0]),
		active_player: if deconstructed_fen[1] == "w" {Player::White} else {Player::Black},
		castle_rights: deconstructed_fen[2],
		enpassant_square: deconstructed_fen[3],
	};
	println!("{:?}", boardstate);
}


fn main() {

	// User FEN input
	// print!("Enter an fen: ");
	// let input_fen: String = read!("{}\n");
	// let fen = &input_fen[..];

	let mut fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	//strips turn info from the FEN
	fen = &fen[..(fen.len() - 4)];

	deconstruct_fen(fen);
	
}
