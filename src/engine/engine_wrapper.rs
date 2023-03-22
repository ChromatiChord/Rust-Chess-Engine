use crate::config::{self, PieceActionTrigger, BoardState, AvailablePieceMoves, Agent, Player, Piece};
use crate::piece_movement;
use crate::piece_movement::piece_movement_brains::get_available_moves_from_state;

pub fn evaluation_position(
    board_state: &BoardState, 
    available_moves: Option<Vec<AvailablePieceMoves>>, 
    depth: i8,  
    alpha: i16, beta: i16, 
    agent: Agent) -> i16 {

    if depth == 0 {
        // evaluate position and return 
        return evaluate_position(board_state);
    }

    // ignoring alpha beta for now

    // check if available moves is empty:
    // if so, this is the first iteration
    let available_moves = match available_moves {
        Some(moves) => moves,
        None => get_available_moves_from_state(board_state, board_state.active_player),
    };

    // vector for storing the results of available positions
    let mut position_results: Vec<i16> = Vec::new();

    // for move in available moves:
    for mv in &available_moves {
        // create a new boardstate with the updated stuff (update_board_with_new_params())
        let new_board_state = update_board_with_new_params(board_state, mv);

        let enemy_player = match board_state.active_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        let enemy_agent = match agent {
            Agent::Max => Agent::Min,
            Agent::Min => Agent::Max,
        };

        let enemy_available_moves = get_available_moves_from_state(&new_board_state, enemy_player);

        let cur_king_coords = get_king_coords(&board_state);

        // for move in enemy_moves:
        for enemy_mv in &enemy_available_moves {
            // if king pos is NOT in available moves (accounts for check):
            if !king_pos_in_moves(&new_board_state, enemy_mv) {
                // send that data into a new evaluation_position(depth - 1, enemy_moves) function
                let result = evaluation_position(&new_board_state, Some(enemy_available_moves), depth - 1, alpha, beta, enemy_agent);
                // append result to vector
                position_results.push(result);
            }
        }
    }

    // if Agent::Min, return smallest value from vector
    // else if Agent::Max, return largest value from vector
    match agent {
        Agent::Max => *position_results.iter().max().unwrap_or(&0),
        Agent::Min => *position_results.iter().min().unwrap_or(&0),
    }
}

fn evaluate_position(board_state: &BoardState) -> i16 {
    0
}

fn update_board_with_new_params(board_state: &BoardState, piece_movement: AvailablePieceMoves) -> BoardState{
    *board_state.clone()
}

// get coords of the king
fn get_king_coords(board_state: &BoardState) {
    match board_state.active_player {
        Player::White => board_state.white_pieces.iter().find(|piece_info| piece_info.piece_type == Piece::King).unwrap().square,
        Player::Black => board_state.black_pieces.iter().find(|piece_info| piece_info.piece_type == Piece::King).unwrap().square,
    }; 
}