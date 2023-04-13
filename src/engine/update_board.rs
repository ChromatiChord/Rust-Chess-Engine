use crate::config::{self, AvailablePieceMove, BoardState, Agent, Player, Piece, PieceInfo, SpecialAction};

pub fn update_board_with_new_params(
  board_state: &BoardState, 
  new_move: &AvailablePieceMove) -> BoardState {
  
  let mut new_board_state = board_state.clone();

  // // check piece type
  // // if pawn and correct square, promote
  // if new_move.piece_info.piece == Piece::Pawn {
  //   // check if pawn is at end of board
  //   if new_move.new_pos.1 == 0 || new_move.new_pos.1 == 7 {
  //     new_move.piece_info.piece = Piece::Queen;
  //   }
  // }
  //   // check if piece king, disable castle rights
  // else if new_move.piece_info.piece == Piece::King {
  //   new_board_state = new_board_state.disable_castle_rights(new_move.piece_info.agent);
  // }
  //   // if piece rook, disable side castle rights
  // else if new_move.piece_info.piece == Piece::Rook {
  //   if new_move.new_pos.0 == 0 {
  //     new_board_state = new_board_state.disable_castle_rights(new_move.piece_info.agent);
  //   }
  //   else if new_move.new_pos.0 == 7 {
  //     new_board_state = new_board_state.disable_castle_rights(new_move.piece_info.agent);
  //   }
  // }
    
  // // check special action:
  // match new_move.special_action {
  //   SpecialAction::EnpassantGenerate => {
  //     // add enpassant square
  //     new_board_state = new_board_state.add_enpassant_square(new_move.new_pos);
  //   },
  //   SpecialAction::EnpassantAttack => {
  //     // remove enpassant square, remove piece in enemy list from correct square
  //   },
  //   SpecialAction::CastleShort => {
  //     // remove piece at correct square, add rook to correct square (short)
  //   },
  //   SpecialAction::CastleLong => {
  //     // remove piece at correct square, add rook to correct square (long)
  //   },
  //   SpecialAction::Capture => {
  //     // remove piece at newpos from eneemy list
  //   },
  //   _ => ()
  // }
    
  // move piece to new square
  // remove piece from old square

  // return new board state
    
    
  new_board_state
}