use super::out_of_bounds::out_of_bounds;

use crate::config::{self, PieceActionTrigger, CastleRights, SpecialAction, Player};


pub fn get_king_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>, castle_rights: CastleRights, team: Player) -> (Vec<(i8, i8)>, Vec<PieceActionTrigger>){
    
    let king_movement: Vec<(i8, i8)> = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1)
    ];

    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    let mut special_possible_squares: Vec<PieceActionTrigger> = Vec::new();

    for movement in king_movement {
        let rank = square.0 + movement.0;
        let file = square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
            continue;
        } else if occupied_enemy.contains(&coordinates) {
            special_possible_squares.push(PieceActionTrigger { 
                new_square: coordinates, 
                special_action: SpecialAction::Capture });
        } else {
            possible_squares.push((rank,file));
        }
    }

    match team {
        Player::White => {
            if castle_rights.white_short == true {
                special_possible_squares.push(PieceActionTrigger { 
                    new_square: (7, 6), 
                    special_action: SpecialAction::CastleShort }
                );
            }
            if castle_rights.white_long == true {
                special_possible_squares.push(PieceActionTrigger { 
                    new_square: (7, 2), 
                    special_action: SpecialAction::CastleLong }
                );
            }
        },
        Player::Black => {
            if castle_rights.black_short == true {
                special_possible_squares.push(PieceActionTrigger { 
                    new_square: (0, 6), 
                    special_action: SpecialAction::CastleShort }
                );
            }
            if castle_rights.black_long == true {
                special_possible_squares.push(PieceActionTrigger { 
                    new_square: (0, 2), 
                    special_action: SpecialAction::CastleLong }
                );
            }
        }
    }

    (possible_squares, special_possible_squares)
}