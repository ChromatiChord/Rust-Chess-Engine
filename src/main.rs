#![allow(dead_code, unused)]

use regex::Regex;
use text_io::read;
mod config;

mod fen;
use fen::fen_construct::fen_construct;
use fen::fen_deconstruct::fen_deconstruct;

mod engine;
use engine::engine_wrapper::get_number_of_moves;

mod piece_movement;
// use piece_movement::piece_movement_brains::get_available_moves;

use crate::config::Agent;

mod evaluation;

fn main() {
    let debug = true;
    let mut input_fen = "";

    if debug {
        input_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
    } else {
        // User FEN input
        print!("Enter an fen: ");
        let input_fen: String = read!("{}\n");
    }

    let mut fen: &str = &input_fen[..];
    if !is_valid_fen(input_fen) {
        panic!("Input is not a valid FEN!");
    }
    //strips turn info and repeat count from the FEN
    let turn_info = &fen[fen.len() - 3..fen.len()];
    fen = &fen[..(fen.len() - 4)];

    let budget = 3;
    let board = fen_deconstruct(fen);

    let (depth, alpha, beta) = (2, 1, 1);

    let num_moves = get_number_of_moves(&board, None, depth, alpha, beta, Agent::Max);
    println!("{:?} moves at depth {:?}", num_moves, depth);
    println!("Done!");
}

fn is_valid_fen(fen: &str) -> bool {
    let fen_regex = Regex::new(r"^([rnbqkpRNBQKP1-8]+/){7}([rnbqkpRNBQKP1-8]+) (w|b) ((K?Q?k?q?|-) (-|[abcdefgh][36]) \d+ \d+)$").unwrap();
    fen_regex.is_match(fen)
}
