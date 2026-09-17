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
    // Get who's turn it is
    pub fn is_white_turn(&self) -> bool {
        self.is_white_turn
    }
    // Turn text
    pub fn turn_text(&self) -> &str {
        if self.is_white_turn() {
            return "White's turn";
        }
        else {
            return "Black's turn";
        }
    }

    // Gets a start and stop square
    // "Checks" if it's valid and executes.
    pub fn make_move(&mut self, start: &str, stop: &str) {
        let start_square = Square::square_from_notation_str(start).unwrap();
        let stop_square = Square::square_from_notation_str(stop).unwrap();

        // Checks that the piece being moved is the same as the color of who's turn it is
        let piece = match self.board_state.get_piece_square(&start_square) {
            Some(piece) => piece,
            None => return,
        };
        if piece.is_white() != self.is_white_turn {
            return;
        }

        if Move::check_move(&self.board_state, &start_square, &stop_square) {
            self.board_state.move_piece_square(&start_square, &stop_square);
            self.is_white_turn = !self.is_white_turn;
        }
    }

}