use crate::config::{self, PieceActionTrigger, BoardState, AvailablePieceMoves, Side};

pub fn evaulation_position(
    board_state: &BoardState, 
    available_moves: Option<Vec<AvailablePieceMoves>>, 
    depth: i8,  
    alpha: i16, beta: i16, 
    team: Side) {
    println!("ENGINE!");

    // if depth == 0:
        // evaluate position and return 

    // ignoring alpha beta for now

    // check if available moves is empty:
        // if so, generate available moves
    
    // make new vector: Vec<i16> for storing the results of available positions

    // for move in available moves:
        // create a new boardstate with the updated stuff (update_board_with_new_params())
        // get available enemy_moves from this new boardstate

        // for move in enemy_moves:
        // if king pos is NOT in available moves (accounts for check):
            // send that data into a new evaulation_position(depth - 1, enemy_moves) function
            // append result to vector

    // if      Side::Enemy, return smallest value from vector
    // else if Side::Own, return largest value from vector

}