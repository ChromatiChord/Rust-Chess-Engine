use crate::config;
use config::Player::White;
use config::Player::Black;

use crate::config::{AvailablePieceMove, PieceInfo, CastleRights, SpecialAction, Player};

pub fn get_pawn_moves(piece_info: &PieceInfo, 
occupied_self: Vec<(i8, i8)>, 
occupied_enemy: Vec<(i8, i8)>, 
enpassant_square: Option<(i8, i8)>, 
active_player: Player) -> 
Vec<AvailablePieceMove> {
    
    let mut possible_squares: Vec<AvailablePieceMove> = Vec::new();

    let rank = piece_info.square.0;
    let file = piece_info.square.1;
    
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

    let mut castle_action = if rank == (double_rank - direction) {
        vec![SpecialAction::Promote]
    } else {
        vec![]
    };

    // check if pawn can jump 1 square ahead
    let one_space_new_square = (rank + direction, file);
    let mut one_ahead_occupied = true;
    if !occupied_self.contains(&one_space_new_square) && !occupied_enemy.contains(&one_space_new_square) {
        possible_squares.push(AvailablePieceMove {
                piece: *piece_info,
                new_square: one_space_new_square,
                special_action: Some(castle_action.clone())
            });
        one_ahead_occupied = false;
    } 
    
    // check if pawn can jump 2 spaces
    if rank == double_rank {
        let two_space_new_square = (rank + direction * 2, file);
        if !occupied_self.contains(&two_space_new_square) && !occupied_enemy.contains(&two_space_new_square) {
            if !one_ahead_occupied {
                possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: two_space_new_square,
                    special_action: Some(vec![SpecialAction::EnpassantGenerate])
                });
            }
        }
    }

    // diagonal capture check
    if occupied_enemy.contains(&(rank + direction, file + 1)) {
        possible_squares.push(AvailablePieceMove {
            piece: *piece_info,
            new_square: (rank + direction, file + 1),
            special_action: Some([&castle_action[..], &[SpecialAction::Capture]].concat())
        });
    }
    if occupied_enemy.contains(&(rank + direction, file - 1)) {
        possible_squares.push(AvailablePieceMove {
            piece: *piece_info,
            new_square: (rank + direction, file - 1),
            special_action: Some([&castle_action[..], &[SpecialAction::Capture]].concat())
        });
    }

    // en passant check
    match enpassant_square {
        Some(en_square) => {
            if en_square ==  (rank + direction, file + 1) {
                possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (rank + direction, file + 1),
                    special_action: Some(vec![SpecialAction::EnpassantAttack])
                });
            }
            else if en_square ==  (rank + direction, file - 1)  {
                possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (rank + direction, file - 1),
                    special_action: Some(vec![SpecialAction::EnpassantAttack])
                });
            }
        },
        None => ()
    }
    possible_squares
}