use crate::config::{self, PieceActionTrigger, BoardState, Agent, Player, Piece, PieceInfo, SpecialAction};

pub fn update_board_with_new_params(
  board_state: &BoardState, 
  piece_info: PieceInfo, 
  new_square: (i8, i8), 
  action: Option<SpecialAction>) -> BoardState {

  board_state.clone()
}