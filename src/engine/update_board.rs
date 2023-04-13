use crate::config::{self, AvailablePieceMove, BoardState, Agent, Player, Piece, PieceInfo, SpecialAction};

pub fn update_board_with_new_params(
  board_state: &BoardState, 
  new_move: &AvailablePieceMove) -> BoardState {
  board_state.clone()
}