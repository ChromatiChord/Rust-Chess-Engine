use core::panic;

use crate::config::{self, BoardState, AvailablePieceMove, Agent, Player, Piece, PieceInfo, SpecialAction};
use crate::piece_movement;
use crate::piece_movement::piece_movement_brains::get_available_moves_from_state;
use super::update_board::update_board_with_new_params;
use super::helper_functions::is_king_in_check;

pub fn get_number_of_moves(
    board_state: &BoardState, 
    available_moves: Option<&Vec<AvailablePieceMove>>, 
    depth: i8,  
    alpha: i16, beta: i16, 
    agent: Agent) -> i16 {

    if depth == 0 {
        // evaluate position and return 
        return 1 as i16;
    }

    // ignoring alpha beta for now

    // check if available moves is empty:
    // if so, this is the first iteration
    let binding = get_available_moves_from_state(&board_state); 
    let available_moves = match available_moves {
        Some(moves) => &*moves,
        None => &binding,
    };
    // for mv in available_moves {
    //     println!("{:?}", mv);
    // }
    let mut position_results = vec![];
    
    for new_move in available_moves {
        let new_board_state = update_board_with_new_params(board_state, new_move.clone());
        
        let enemy_player = board_state.active_player;
        
        let enemy_agent = match agent {
            Agent::Max => Agent::Min,
            Agent::Min => Agent::Max,
        };
        
        let enemy_available_moves = get_available_moves_from_state(&new_board_state);
        
        // for move in enemy_moves:
        for enemy_mv in &enemy_available_moves {
            // if king pos is NOT in available moves (accounts for check):
            if !is_king_in_check(&new_board_state, &enemy_available_moves) {
                // send that data into a new get_number_of_moves(depth - 1, enemy_moves) function
                let result = get_number_of_moves(&new_board_state, Some(&enemy_available_moves), depth - 1, alpha, beta, enemy_agent);
                // append result to vector
                position_results.push(result);
            }
        }
    }
    position_results.iter().sum()

}


    // // if Agent::Min, return smallest value from vector
    // // else if Agent::Max, return largest value from vector
    // // match agent {
    // //     Agent::Max => *position_results.iter().sum().unwrap_or(&0),
    // //     Agent::Min => *position_results.iter().min().unwrap_or(&0),
    // // }
    // // if Agent::Min, return smallest value from vector
    // // else if Agent::Max, return largest value from vector
    // 1