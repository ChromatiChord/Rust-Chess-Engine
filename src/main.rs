#![allow(dead_code, unused)]

// use text_io::read;
mod config;
mod fen;
use fen::fen_construct::fen_construct;
use fen::fen_deconstruct::fen_deconstruct;

mod piece_movement;
use piece_movement::piece_movement_brains::get_available_moves;

mod evaluation;

fn main() {
	// User FEN input
	// print!("Enter an fen: ");
	// let input_fen: String = read!("{}\n");
	// let fen = &input_fen[..];

	let budget = 5_000_000;

	let mut fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - e3 0 1";

	//strips turn info and repeat count from the FEN
	let turn_info = &fen[fen.len() - 3..fen.len()];
	fen = &fen[..(fen.len() - 4)];

	let board = fen_deconstruct(fen);

	let debug_selflist: Vec<(i8, i8)> = vec![(4, 4)];
	let debug_enemylist: Vec<(i8, i8)> = vec![(2, 0)];
	
	// for i in 1..1_0{
	// }
	
	for piece in &board.white_pieces {
		println!("{:?} {:?}: {:?}", piece.owner, piece.piece_type, get_available_moves(&piece.piece_type, &piece.owner, &piece.square, &board.occupied_white, &board.occupied_black, board.enpassant_square, &"right"));
	}

	// println!("{:?} {:?}: {:?}", config::Player::White, config::Piece::Rook, get_available_moves(&config::Piece::Rook, &config::Player::White, &(1,2), &debug_selflist, &debug_enemylist, Some((8, 8))));

	//  args: ( piece, player, piece_coords, occ_white, occ_black, enpassant_square )
	// let squares = get_available_moves(config::Piece::Knight, config::Player::White, (1,2), debug_selflist, debug_enemylist, (8, 8));

	println!("Done!")
}
