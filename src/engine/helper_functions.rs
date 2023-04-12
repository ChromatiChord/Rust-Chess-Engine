use crate::config::{self, PieceActionTrigger, BoardState, AvailablePieceMoves, Agent, Player, Piece, PieceInfo, SpecialAction};

// check if king is in check
pub fn is_king_in_check(board_state: &BoardState, enemy_available_moves: &Vec<AvailablePieceMoves>) -> bool {
  let king_coords = match board_state.active_player {
      Player::White => board_state.white_pieces.iter().find(|piece_info| piece_info.piece_type == Piece::King).unwrap().square,
      Player::Black => board_state.black_pieces.iter().find(|piece_info| piece_info.piece_type == Piece::King).unwrap().square,
  }; 

  for piece_moves in enemy_available_moves {
      for available_move in &piece_moves.available_moves {
          if *available_move == king_coords {
              return true;
          }
      }
      for available_move in &piece_moves.special_actions {
          if available_move.new_square == king_coords {
              return true;
          }
      }
  }

  return false;
}