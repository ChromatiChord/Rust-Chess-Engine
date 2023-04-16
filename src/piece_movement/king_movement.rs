use super::out_of_bounds::out_of_bounds;

use crate::config::{self, AvailablePieceMove, PieceInfo, CastleRights, SpecialAction, Player};

pub fn get_king_moves(piece_info: &PieceInfo, occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>, castle_rights: CastleRights, team: Player) -> 
Vec<AvailablePieceMove> {
    
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

    let mut possible_squares: Vec<AvailablePieceMove> = Vec::new();

    for movement in king_movement {
        let rank = piece_info.square.0 + movement.0;
        let file = piece_info.square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
            continue;
        } else if occupied_enemy.contains(&coordinates) {
            possible_squares.push(AvailablePieceMove {
                piece: *piece_info,
                new_square: coordinates,
                special_action: Some(vec![SpecialAction::Capture, SpecialAction::DisableCastleShort, SpecialAction::DisableCastleLong])
            });
        } else {
            possible_squares.push(AvailablePieceMove {
                piece: *piece_info,
                new_square: coordinates,
                special_action: Some(vec![SpecialAction::DisableCastleShort, SpecialAction::DisableCastleLong])
            });
        }
    }

    match team {
        Player::White => {
            if castle_rights.white_short == true && check_castle_path_free(&occupied_self, &occupied_enemy, team, "short") {
                possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (7, 6),
                    special_action: Some(vec![SpecialAction::CastleShort])
                });
            }
            if castle_rights.white_long == true && check_castle_path_free(&occupied_self, &occupied_enemy, team, "long") {
                possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (7, 2),
                    special_action: Some(vec![SpecialAction::CastleLong])
                });
            }
        },
        Player::Black => {
            if castle_rights.black_short == true && check_castle_path_free(&occupied_self, &occupied_enemy, team, "short") {
                    possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (0, 6),
                    special_action: Some(vec![SpecialAction::CastleShort])
                });
            }
            if castle_rights.black_long == true && check_castle_path_free(&occupied_self, &occupied_enemy, team, "long") {
                    possible_squares.push(AvailablePieceMove {
                    piece: *piece_info,
                    new_square: (0, 2),
                    special_action: Some(vec![SpecialAction::CastleLong])
                });
            }
        }
    }

    possible_squares
}

fn check_castle_path_free(occupied_self: &Vec<(i8, i8)>, occupied_enemy: &Vec<(i8, i8)>, team: Player, direction: & str) -> bool {
    if direction == "short" {
        match Player::White {
            Player::White => {
                if occupied_self.contains(&(7, 5)) || occupied_self.contains(&(7, 6)) || occupied_enemy.contains(&(7, 5)) || occupied_enemy.contains(&(7, 6)) {
                    return false;
                }
            },
            Player::Black => {
                if occupied_self.contains(&(0, 5)) || occupied_self.contains(&(0, 6)) || occupied_enemy.contains(&(0, 5)) || occupied_enemy.contains(&(0, 6)) {
                    return false;
                }
            }
        }
    } else if direction == "long" {
        match Player::White {
            Player::White => {
                if occupied_self.contains(&(7, 3)) || occupied_self.contains(&(7, 2)) || occupied_enemy.contains(&(7, 3)) || occupied_enemy.contains(&(7, 2)) {
                    return false;
                }
            },
            Player::Black => {
                if occupied_self.contains(&(0, 3)) || occupied_self.contains(&(0, 2)) || occupied_enemy.contains(&(0, 3)) || occupied_enemy.contains(&(0, 2)) {
                    return false;
                }
            }
        }
    }

    return true;
}