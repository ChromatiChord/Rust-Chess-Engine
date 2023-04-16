use crate::config::{
    self, Agent, AvailablePieceMove, BoardState, Piece, PieceInfo, Player, SpecialAction,
};

pub fn update_board_with_new_params(
    board_state: &BoardState,
    mut new_move: AvailablePieceMove,
) -> BoardState {
    let mut new_board_state = board_state.clone();
    
    // check special action:
    match new_move.special_action {
        Some(special_actions) => {
            for action in special_actions {
                match action {
                    SpecialAction::EnpassantGenerate => {
                        // add enpassant square ✓
                        let en_square = get_enpassant_attack_square(&board_state.active_player, new_move.new_square);
                        new_board_state.enpassant_square = Some(en_square);
                    },
                    SpecialAction::EnpassantAttack => {
                        // remove enpassant square, remove piece in enemy list from correct square ✓
                        new_board_state.enpassant_square = None;
                        new_board_state = remove_piece_at_square_side_agnostic(new_board_state.clone(), get_enpassant_attack_square(&board_state.active_player, new_move.new_square));

                    },
                    SpecialAction::CastleShort => {
                        // remove piece at correct square, add rook to correct square (short)
                        match new_board_state.active_player {
                            Player::White => {
                                // change rook square at 7, 7 to 7, 5
                                new_board_state = remove_piece_at_square(new_board_state.clone(), (7, 7), Player::White, false);
                                new_board_state.white_pieces.push(PieceInfo {
                                    square: (7, 5),
                                    piece_type: Piece::Rook,
                                    piece_value: 5,
                                    owner: Player::White,
                                });
                            }
                            Player::Black => {
                                // change rook square at 0, 7 to 0, 5
                                new_board_state = remove_piece_at_square(new_board_state.clone(), (0, 7), Player::Black, false);
                                new_board_state.black_pieces.push(PieceInfo {
                                    square: (0, 5),
                                    piece_type: Piece::Rook,
                                    piece_value: 5,
                                    owner: Player::Black,
                                });
                            }
                        }
                    },
                    SpecialAction::CastleLong => {
                        // remove piece at correct square, add rook to correct square (long)
                        match new_board_state.active_player {
                            Player::White => {
                                // change rook square at 7, 0 to 7, 3
                                new_board_state = remove_piece_at_square(new_board_state.clone(), (7, 0), Player::White, false);
                                new_board_state.white_pieces.push(PieceInfo {
                                    square: (7, 3),
                                    piece_type: Piece::Rook,
                                    piece_value: 5,
                                    owner: Player::White,
                                });
                            }
                            Player::Black => {
                                // change rook square at 0, 0 to 0, 3
                                new_board_state = remove_piece_at_square(new_board_state.clone(), (0, 0), Player::Black, false);
                                new_board_state.black_pieces.push(PieceInfo {
                                    square: (0, 3),
                                    piece_type: Piece::Rook,
                                    piece_value: 5,
                                    owner: Player::Black,
                                });
                            }
                        }
                    },
                    SpecialAction::Capture => {
                        // remove piece at newpos from enemy list ✓
                        new_board_state = remove_piece_at_square(new_board_state.clone(), new_move.new_square, new_board_state.active_player, true);

                    },
                    SpecialAction::DisableCastleLong => {
                        // ✓
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
                        // ✓
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
                        // ✓
                        new_move.piece.piece_type = Piece::Queen;
                    },
                    _ => (),
                }
            }
        }
        None => (),
    }

    // move piece to new square
    new_board_state = move_piece(new_board_state.clone(), &new_move.piece, new_move.piece.square, new_move.new_square, new_board_state.active_player, false);

    // switch active player
    new_board_state.active_player = match new_board_state.active_player {
        Player::White => Player::Black,
        Player::Black => Player::White,
    };

    // return new board state
    new_board_state
}

fn move_piece(mut new_board_state: BoardState, new_move_piece: &PieceInfo, old_square: (i8, i8), new_square: (i8, i8), active_player: Player, remove_enemy: bool) -> BoardState {
    new_board_state = remove_piece_at_square(new_board_state.clone(), old_square, active_player, remove_enemy);
    match active_player {
        Player::White => {
            new_board_state.white_pieces.push(PieceInfo {
                square: new_square,
                piece_type: new_move_piece.piece_type,
                piece_value: new_move_piece.piece_value,
                owner: new_move_piece.owner,
            });
            new_board_state.occupied_white.push(new_square);
        },
        Player::Black => {
            new_board_state.black_pieces.push(PieceInfo {
                square: new_square,
                piece_type: new_move_piece.piece_type,
                piece_value: new_move_piece.piece_value,
                owner: new_move_piece.owner,
            });
            new_board_state.occupied_white.push(new_square);
        },
    }
    
    new_board_state
}

fn remove_piece_at_square(mut board_state: BoardState, coords: (i8, i8), active_player: Player, remove_enemy: bool) -> BoardState{
    if remove_enemy {
        match active_player {
            Player::White => {
                board_state.black_pieces.retain(|piece| piece.square != coords);
                board_state.occupied_black.retain(|square| square != &coords);
            },
            Player::Black => {
                board_state.white_pieces.retain(|piece| piece.square != coords);
                board_state.occupied_white.retain(|square| square != &coords);
            }
        }
    } else {
        match active_player {
            Player::White => {
                board_state.white_pieces.retain(|piece| piece.square != coords);
                board_state.occupied_white.retain(|square| square != &coords);
            },
            Player::Black => {
                board_state.black_pieces.retain(|piece| piece.square != coords);
                board_state.occupied_black.retain(|square| square != &coords);
            }
        }
    }

    board_state
}

fn remove_piece_at_square_side_agnostic(mut board_state: BoardState, coords: (i8, i8)) -> BoardState{
    // delete piece at coords from player's list
    board_state.white_pieces.retain(|piece| piece.square != coords);
    board_state.black_pieces.retain(|piece| piece.square != coords);
    // delete coords from occupied list
    board_state.occupied_white.retain(|square| square != &coords);
    board_state.occupied_black.retain(|square| square != &coords);
    
    board_state
}

fn get_enpassant_attack_square(player: &Player, final_en_square: (i8, i8)) -> (i8, i8) {
    match player {
        Player::White => (final_en_square.0 + 1, final_en_square.1),
        Player::Black => (final_en_square.0 - 1, final_en_square.1),
    }
}