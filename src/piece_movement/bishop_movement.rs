use crate::config;
use config::PieceMovementTrigger;
use config::SpecialAction;


use super::out_of_bounds::out_of_bounds;

pub fn get_bishop_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) ->
(Vec<(i8, i8)>, Vec<PieceMovementTrigger>) {
    let bishop_movement: Vec<(i8, i8)> = vec![
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1)
    ];
    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    let mut special_possible_squares: Vec<PieceMovementTrigger> = Vec::new();

    for movement in bishop_movement {
        let mut rank = square.0;
        let mut file = square.1;
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
                special_possible_squares.push(PieceMovementTrigger { 
                    new_square: coordinates, 
                    special_action: SpecialAction::Capture });
                    stop = true;
            } else {
                possible_squares.push((rank,file));
            }

        }
    }
    (possible_squares, special_possible_squares)
} 