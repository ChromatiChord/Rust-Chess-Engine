use crate::config;
use config::Player;
use config::Player::White;
use config::Player::Black;

use config::PieceMovementTrigger;
use config::SpecialAction;

pub fn get_pawn_moves(square: (i8, i8), 
occupied_self: Vec<(i8, i8)>, 
occupied_enemy: Vec<(i8, i8)>, 
enpassant_square: Option<(i8, i8)>, 
active_player: Player) -> (Vec<(i8, i8)>, Vec<PieceMovementTrigger>) {
    
    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    let mut special_possible_squares: Vec<PieceMovementTrigger> = Vec::new();
    
    // check which rank the pawn is allowed to dash on
    let double_rank = match active_player {
        White => 6,
        Black => 1
    };
    // check which direction the pawn is meant to be moving
    let direction = match active_player {
        White => -1,
        Black => 1
    };
    
    // check if pawn can jump 1 square ahead
    let one_space_new_square = (square.0 + direction, square.1);
    let mut one_ahead_occupied = true;
    if !occupied_self.contains(&one_space_new_square) && !occupied_enemy.contains(&one_space_new_square) {
        possible_squares.push(one_space_new_square);
        one_ahead_occupied = false;
    } 
    
    // check if pawn can jump 2 spaces
    if square.0 == double_rank {
        let two_space_new_square = (square.0 + direction * 2, square.1);
        if !occupied_self.contains(&two_space_new_square) && !occupied_enemy.contains(&two_space_new_square) {
            if !one_ahead_occupied {
                special_possible_squares.push(PieceMovementTrigger {
                    new_square: two_space_new_square,
                    special_action: SpecialAction::EnpassantGenerate
                });
            }
        }
    }

    // diagonal capture check
    if occupied_enemy.contains(&(square.0 + direction, square.1 + 1)) {
        special_possible_squares.push(PieceMovementTrigger {
            new_square: (square.0 + direction, square.1 + 1),
            special_action: SpecialAction::Capture
        });
    }
    if occupied_enemy.contains(&(square.0 + direction, square.1 - 1)) {
        special_possible_squares.push(PieceMovementTrigger {
            new_square: (square.0 + direction, square.1 - 1),
            special_action: SpecialAction::Capture
        });
    }

    // en passant check
    match enpassant_square {
        Some(en_square) => {
            if en_square ==  (square.0 + direction, square.1 + 1) {
                special_possible_squares.push(PieceMovementTrigger {
                    new_square: (square.0 + direction, square.1 + 1),
                    special_action: SpecialAction::EnpassantAttack
                });
            }
            else if en_square ==  (square.0 + direction, square.1 - 1)  {
                special_possible_squares.push(PieceMovementTrigger {
                    new_square: (square.0 + direction, square.1 - 1),
                    special_action: SpecialAction::EnpassantAttack
                });
            }
        },
        None => ()
    }
    
    (possible_squares, special_possible_squares)
}