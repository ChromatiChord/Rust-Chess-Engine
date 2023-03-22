use crate::config;
use config::PieceMovementTrigger;
use config::SpecialAction;
use super::out_of_bounds::out_of_bounds;

pub fn get_knight_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) -> 
(Vec<(i8, i8)>, Vec<PieceMovementTrigger>) {

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

    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    let mut special_possible_squares: Vec<PieceMovementTrigger> = Vec::new();

    for movement in knight_movement {
        let rank = square.0 + movement.0;
        let file = square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
            continue;
        } else if occupied_enemy.contains(&coordinates) {
            special_possible_squares.push(PieceMovementTrigger { 
                new_square: coordinates, 
                special_action: SpecialAction::Capture });
        } else {
            possible_squares.push((rank,file));
        }
    }
    (possible_squares, special_possible_squares)
}