use crate::config::{self, SpecialAction, AvailablePieceMove, PieceInfo};
use super::out_of_bounds::out_of_bounds;

pub fn get_knight_moves(piece_info: PieceInfo, occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) -> 
Vec<AvailablePieceMove> {

    let knight_movement: Vec<(i8, i8)> = vec![
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, -1),
        (2, 1),
        (-1, -2),
        (1, -2)
    ];

    let mut possible_squares: Vec<AvailablePieceMove> = Vec::new();

    for movement in knight_movement {
        let rank = piece_info.square.0 + movement.0;
        let file = piece_info.square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
            continue;
        } else if occupied_enemy.contains(&coordinates) {
            possible_squares.push(AvailablePieceMove {
                piece: piece_info,
                new_square: coordinates,
                special_action: Some(SpecialAction::Capture)
            });
        } else {
            possible_squares.push(AvailablePieceMove {
                piece: piece_info,
                new_square: coordinates,
                special_action: None
            });
        }
    }
    possible_squares
}