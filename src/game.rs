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
    // Prints relevant error depending on where move fails/ why is invalid
    pub fn make_move(&mut self, start: &str, stop: &str) -> Result<(), String> {
        
        // Can't make square from start string
        let start_square = match Square::square_from_notation_str(start) {
            Some(square) => square,
            None => return Err(format!("Invalid Starting Square [{}]", start)),
        };
        // Can't make square from stop string
        let stop_square = match Square::square_from_notation_str(stop) {
            Some(square) => square,
            None => return Err(format!("Invalid Stopping Square [{}]", stop)),
        };

        // No piece to move
        // Mark th piece as mutable for later
        let mut piece = match self.board_state.get_piece_square(&start_square) {
            Some(piece) => piece,
            None => return Err(format!("Starting Square Empty [{}]", start)),
        };

        // Checks that the piece being moved is the same as the color of who's turn it is
        if piece.is_white() != self.is_white_turn {
            return Err(format!("Not your piece [{}], it's {}", start, self.turn_text()));
        }

        //
        if !Move::check_move(&self.board_state, &start_square, &stop_square) {
            return Err(format!("Illegal move for that piece {:#?} to move from {} to {}",self.board_state.get_piece_square(&start_square) ,start, stop));
        }
        
        // If no errors move is valid, do move and change turn
        // Mark piece as having moved
        piece.has_moved_true();
        self.board_state.set_piece_square(&start_square, Some(piece));
        self.board_state.move_piece_square(&start_square, &stop_square);

        self.is_white_turn = !self.is_white_turn;
        Ok(())
    }

}