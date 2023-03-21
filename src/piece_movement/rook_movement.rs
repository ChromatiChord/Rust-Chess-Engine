use crate::config::PieceMovementTrigger;

use super::out_of_bounds::out_of_bounds;

pub fn get_rook_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) ->
(Vec<(i8, i8)>, Option<PieceMovementTrigger>) {

    let rook_movement: Vec<(i8, i8)> = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1)
    ];
    
    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    for movement in rook_movement {
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
                possible_squares.push(coordinates);
                stop = true;
            } else {
                possible_squares.push((rank,file));
            }

        }
    }
    (possible_squares, None)
} 