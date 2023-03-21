use super::out_of_bounds::out_of_bounds;
use crate::config::PieceMovementTrigger;
use crate::config::CastleRights;


pub fn get_king_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>, castle_rights: CastleRights) -> (Vec<(i8, i8)>, Vec<PieceMovementTrigger>){
    
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
    let mut special_possible_squares: Vec<PieceMovementTrigger> = Vec::new();

    for movement in king_movement {
        let rank = square.0 + movement.0;
        let file = square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if out_of_bounds(rank, file) || occupied_self.contains(&coordinates) {
            continue;
        } else if occupied_enemy.contains(&coordinates) {
            possible_squares.push(coordinates);
            continue;
        } else {
            possible_squares.push((rank,file));
        }
    }
    (possible_squares, special_possible_squares)
}