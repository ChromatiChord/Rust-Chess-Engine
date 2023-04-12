use crate::config::{self, PieceActionTrigger, SpecialAction, AvailablePieceMove, PieceInfo};

use super::out_of_bounds::out_of_bounds;

pub fn get_bishop_moves(piece_info: PieceInfo, occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) ->
Vec<AvailablePieceMove> {
    let bishop_movement: Vec<(i8, i8)> = vec![
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1)
    ];
    let mut possible_squares: Vec<AvailablePieceMove> = Vec::new();

    for movement in bishop_movement {
        let mut rank = piece_info.square.0;
        let mut file = piece_info.square.1;
        let rank_iterate = movement.0;
        let file_iterate = movement.1;
        let mut stop = false;

        while !stop {
            rank += rank_iterate;
            file += file_iterate;

            let coordinates = (rank, file);
            // checks if values are out of bounds, or if it's occupied by own pieces
            if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
                stop = true;
            } else if occupied_enemy.contains(&coordinates) {
                possible_squares.push(AvailablePieceMove {
                    piece: piece_info,
                    new_square: coordinates,
                    special_action: Some(SpecialAction::Capture)
                });
                stop = true;
            } else {
                possible_squares.push(AvailablePieceMove {
                    piece: piece_info,
                    new_square: coordinates,
                    special_action: None
                });
            }
        }
    }
    possible_squares
}