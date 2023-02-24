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
	let mut final_board = vec![];
	
	for (index, rank) in ranks.iter().enumerate() {
		let mut new_rank = vec![];
		for file in rank.chars() {
			new_rank.push(get_new_file(file))
		}
		final_board.push(new_rank);
	}

	return final_board;
}

fn get_new_file(file: char) -> Vec<char> {
	let mut new_file = vec![];

	if file.is_numeric() {
		println!("TRUE! {}", file);
		let empty_square_num: i32 = file.parse().unwrap();
		for _ in 0..empty_square_num {
			new_file.push('_');
		}
	} else {
		new_file.push(&file[..]);
	}

	return new_file
}

fn convert_active_player(player: &str) -> Player {
	return if player == "w" {Player::White} else {Player::Black};
}

fn deconstruct_fen(fen: &str) {
	// deconstruct FEN into major components
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	
	let mut boardstate = BoardState {
		board_state: construct_board(deconstructed_fen[0]),
		active_player: convert_active_player(deconstructed_fen[1]),
		castle_rights: deconstructed_fen[2],
		enpassant_square: deconstructed_fen[3],
	};
}

fn remove_turns_fen(fen: &str) -> String {
	let deconstructed_fen: Vec<&str> = fen.split_whitespace().collect();
	let removed_turns: String = deconstructed_fen.concat();
	return removed_turns
}

fn main() {
	// 8/3Pp3/p7/5k1r/BP6/B2pK2p/1p2npP1/1R6 w - - 0 1

	// print!("Enter an fen: ");
	// let input_fen: String = read!("{}\n");
	// let fen = &input_fen[..];
	let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
	print!("{}", fen);
	let big_fen = remove_turns_fen(fen);
	let str_fen = &big_fen[..];
	print!("{}", str_fen);
	// deconstruct_fen(fen);
	
}
