use crate::config::{
    self, Agent, AvailablePieceMove, BoardState, Piece, PieceInfo, Player, SpecialAction,
};

pub fn update_board_with_new_params(
    board_state: &BoardState,
    new_move: &AvailablePieceMove,
) -> BoardState {
    let mut new_board_state = board_state.clone();
    
    // check special action:
    match new_move.special_action {
        Some(special_actions) => {
            for action in special_actions {
                match action {
                    SpecialAction::EnpassantGenerate => {
                        // add enpassant square
                    },
                    SpecialAction::EnpassantAttack => {
                        // remove enpassant square, remove piece in enemy list from correct square
                    },
                    SpecialAction::CastleShort => {
                        // remove piece at correct square, add rook to correct square (short)
                    },
                    SpecialAction::CastleLong => {
                        // remove piece at correct square, add rook to correct square (long)
                    },
                    SpecialAction::Capture => {
                        // remove piece at newpos from eneemy list
                    },
                    SpecialAction::DisableCastleLong => {
                        // disable castle long
                        match new_board_state.active_player {
                            Player::White => {
                                new_board_state.castle_rights.white_long = false;
                            }
                            Player::Black => {
                                new_board_state.castle_rights.black_long = false;
                            }
                        }
                    },
                    SpecialAction::DisableCastleShort => {
                        // disable castle short
                        match new_board_state.active_player {
                            Player::White => {
                                new_board_state.castle_rights.white_short = false;
                            }
                            Player::Black => {
                                new_board_state.castle_rights.black_short = false;
                            }
                        }
                    },
                    SpecialAction::Promote => {
                        new_move.piece.piece_type = Piece::Queen;
                    },
                    _ => (),
                }
            }
        }
        None => (),
    }

    // move piece to new square
    // remove piece from old square

    // return new board state

    new_board_state
}
