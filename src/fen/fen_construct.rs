use crate::config;

pub fn fen_construct(board_state: config::BoardState, turn_info: &str) -> String {
    let active_player = match board_state.active_player {
        config::Player::White => "w",
        config::Player::Black => "b"
    };

    let game_history = format!("{} {} {} {}", 
        active_player, 
        board_state.castle_rights, 
        board_state.enpassant_square, 
        turn_info
    );
    
    let piece_info = get_piece_info(board_state.board_state);

    format!("{} {}", piece_info, game_history)
}

fn get_piece_info(board: Vec<Vec<char>>) -> String{
    let mut final_string = String::from("");
    
    for rank in board {
        let mut underscore_count: u32 = 0;
        let mut rank_string = String::from("");
        for file in rank {
            if file != '_' {
                if underscore_count != 0 {
                    rank_string.push(char::from_digit(underscore_count as u32, 10).unwrap());
                }
                rank_string.push(file);
            } else {
                underscore_count += 1;
            }
        }
        if underscore_count != 0 {
            rank_string.push(char::from_digit(underscore_count as u32, 10).unwrap());
        }
        rank_string.push('/');
        final_string = final_string + &rank_string;
    }

    final_string.pop();
    // println!("{}", final_string);
    final_string

}