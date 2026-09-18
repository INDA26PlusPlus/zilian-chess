use crate::board::{Board, Square};
use crate::pieces::{self, ChessPiece, PieceType};
use crate::moves::Move;

pub struct ChessGame {
    board_state: Board,
    is_white_turn: bool,
    // last_move_passant: Option<Square>,
    // 
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
            is_white_turn,
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

        // If move passes the movement logic check
        if !Move::check_move(&self.board_state, &start_square, &stop_square) {
            return Err(format!("Illegal move for that piece {:#?} to move from {} to {}",self.board_state.get_piece_square(&start_square) ,start, stop));
        }

        // Test move and see if it causes current player to go under check
        let mut test_board = self.board_state;
        test_board.move_piece_square(&start_square, &stop_square);
        if Self::is_in_check(&test_board, self.is_white_turn) {
            return Err(format!("That Move would put you in check!"))
        } 
        
        // If no errors move is valid, do move and change turn
        // Mark piece as having moved
        piece.has_moved_true();
        self.board_state.set_piece_square(&start_square, Some(piece));
        self.board_state.move_piece_square(&start_square, &stop_square);

        self.is_white_turn = !self.is_white_turn;
        Ok(())
    }


    // Find square with king of given color
    pub fn find_king(board: &Board, is_white: bool) -> Option<Square> {
        for file in 0..8 {
            for rank in 0..8 {
                match board.get_piece_file_rank(file, rank) {
                    Some(piece) 
                    => if piece.piece_type() == PieceType::King && piece.is_white() == is_white {
                        return Square::new_square_from_index(file, rank);
                    },
                    None => {}
                }
            }
        }
        return  None;
    }

    // Given a square and a color
    // Find out if the enemy is attacking that square (has LOS)
    pub fn is_attacked(board: &Board, attacked_square: &Square, is_white: bool) -> bool {
        for file in 0..8 {
            for rank in 0..8 {
                // The square we want to see if it has LOS to attacked_square
                let mut checking_square = Square::new_square_from_index(file, rank).unwrap();

                match board.get_piece_file_rank(file, rank) {
                    Some(piece) 
                    => if piece.is_white() == is_white {
                        continue; // Skip if same color
                    },
                    None => continue, // Skip if empty
                }
                if Move::check_move(board, &checking_square, attacked_square) {
                    return true;
                }
            }
        }
        return  false;
    }

    // Combine two previous to find out if in check
    pub fn is_in_check(board: &Board, is_white: bool) -> bool {
        match Self::find_king(board, is_white) {
            Some(king_square) =>
            return Self::is_attacked(board, &king_square, is_white),
            None => return false,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_pieces() {
        // Try to see if we can move a pinned piece
        let board = Board::board_from_strings([
        "....r...",
        "........",
        "........",
        "........",
        "........",
        "........",
        "....B...",
        "....K...",
        ]);

        let mut game = ChessGame::new_game(board, true);
        assert!(!game.make_move("e2", "d3").is_ok());
    }
}