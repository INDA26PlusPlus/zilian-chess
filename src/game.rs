use crate::board::{Board, Square};
use crate::pieces::{ChessPiece, PieceType};
use crate::moves::Move;

pub struct ChessGame {
    board_state: Board,
    is_white_turn: bool,
    // Can probably happen through like a "any legal moves" typa-check
    /* 
    white_check: bool,
    black_check: bool,
    stalemate: bool,
    white_checkmate: bool,
    black_checkmate: bool 
    */
}

impl ChessGame {

    // Starts a game from a given board and player turn.
    pub fn new_game(board_state: Board, is_white_turn: bool) -> Self {
        Self {
            board_state,
            is_white_turn
        }
    }

    // Getter's
    // Get's current board.
    pub fn board(&self) -> &Board {
        &self.board_state
    }

    // Gets a start and stop square
    // "Check's" if it's valid and executes.
    pub fn make_move(&mut self, start: &str, stop: &str) {
        let start_square = Square::square_from_notation_str(start).unwrap();
        let stop_square = Square::square_from_notation_str(stop).unwrap();

        if Move::check_move(&self.board_state, &start_square, &stop_square) {
            self.board_state.move_piece_square(&start_square, &stop_square);
        }
    }

}