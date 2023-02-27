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

	let _range = 5_0;

	let mut fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	//strips turn info and repeat count from the FEN
	let turn_info = &fen[fen.len() - 3..fen.len()];
	fen = &fen[..(fen.len() - 4)];

	let board: config::BoardState = fen_deconstruct(fen);

  let _construct_fen = fen_construct(board, turn_info);
	let one: Vec<(i32, i32)> = vec![(1,1)];
	let two: Vec<(i32, i32)> = vec![(1,1)];

	get_available_moves('b', one.clone(), two.clone());
	get_available_moves('k', one, two);
	println!("Done!");
}
