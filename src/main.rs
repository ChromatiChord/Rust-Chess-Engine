// use text_io::read;
mod config;
mod fen;
use fen::fen_construct::fen_construct;
use fen::fen_deconstruct::fen_deconstruct;

mod piece_movement;
use piece_movement::piece_movement_brains::get_available_moves;


fn main() {
	// User FEN input
	// print!("Enter an fen: ");
	// let input_fen: String = read!("{}\n");
	// let fen = &input_fen[..];

	let budget = 5_000_000;

	let mut fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	//strips turn info and repeat count from the FEN
	let turn_info = &fen[fen.len() - 3..fen.len()];
	fen = &fen[..(fen.len() - 4)];

	let board: config::BoardState = fen_deconstruct(fen);

  let _construct_fen = fen_construct(board, turn_info);
	let selflist: Vec<(i8, i8)> = vec![(1,1), (7,7)];
	let enemylist: Vec<(i8, i8)> = vec![(2,6), (2,2)];

	// let squares = get_available_moves('b', (4,4), selflist.clone(), enemylist.clone());
	// get_available_moves('r', (1,1), one, two);

	// print!("{:?}", squares);
	
	for _ in 0..budget {
		get_available_moves('b', (4,4), selflist.clone(), enemylist.clone());
	}
	print!("Done!")
}
