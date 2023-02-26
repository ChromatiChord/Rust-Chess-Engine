// use text_io::read;
mod config;
mod fen;
use fen::fen_construct::fen_construct;
use fen::fen_deconstruct::fen_deconstruct;

fn main() {
	// User FEN input
	// print!("Enter an fen: ");
	// let input_fen: String = read!("{}\n");
	// let fen = &input_fen[..];

	let RANGE = 1_000;

	let mut fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	//strips turn info and repeat count from the FEN
	let turn_info = &fen[fen.len() - 3..fen.len()];
	fen = &fen[..(fen.len() - 4)];

	let mut board: config::BoardState = fen_deconstruct(fen);

	let construct_fen = fen_construct(board, turn_info);

	// for _ in 0..RANGE {
	// 	let test_board = fen_deconstruct::fen_deconstruct(fen);
	// 	let construct_board = fen_construct::fen_construct(test_board, turn_info);
	// }	
	println!("Done!");
}
