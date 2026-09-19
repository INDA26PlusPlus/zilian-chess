use crate::board::{Board, Square};
use crate::pieces::{PieceType};
use crate::moves::Move;

// Improved error handling for the API
#[derive(Debug)]
pub enum MoveError {
    InvalidNotation, // Notation wrong, like x9
    EmptySquare,     // Tried to move a empty square
    NotYourPiece,    // Tried to move enemy piece
    CantSelfHarm,    // Tried to attack own piece
    IllegalMove,     // Tried an illegal move
    HangsKing,       // Tried a move that self-checks
}

pub struct ChessGame {
    // Current board layout
    board_state: Board,
    // Whos turn it is
    is_white_turn: bool,
    // 
    movement_logic: Move,
}

impl ChessGame {

    // Starts a game from a given board and player turn.
    pub fn new_game(board_state: Board, is_white_turn: bool) -> Self {
        Self {
            board_state,
            is_white_turn,
            movement_logic: Move::new_move(),
        }
    }

    // Starts a new game with standard layout and whites turn
    pub fn new_standard_game() -> Self {
        Self {
            board_state: Board::new_starting_board(),
            is_white_turn: true,
            movement_logic: Move::new_move(),
        }
    }

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

    // Current player check
    pub fn in_check(&self) -> bool {
        self.is_in_check(&self.board_state, self.is_white_turn)
    }

    // Current player checkmate
    pub fn in_checkmate(&self) -> bool {
        self.is_checkmate(&self.board_state, self.is_white_turn)
    }

    // Current player stalemate
    pub fn in_stalemate(&self) -> bool {
        self.is_stalemate(&self.board_state, self.is_white_turn)
    }

    // Tie by fifty-move-rule
    pub fn in_tie(&self) -> bool {
        return self.movement_logic.get_fifty_increment() >= 100;
    }

    pub fn make_move_notation(&mut self, start: &str, stop: &str) -> Result<(), MoveError> {
        self.make_move(
            Square::new_square_from_notation_str(&start),
            Square::new_square_from_notation_str(&stop)
        )
    }

    // Gets a start and stop square
    // Runs checks
    // Prints relevant error depending on where move fails/ why is invalid
    pub fn make_move(&mut self, start: Option<Square>, stop: Option<Square>) -> Result<(), MoveError> {

        // Checks that start square is valid
        let start_square = match start {
            Some(square) => square,
            None => return Err(MoveError::InvalidNotation),
        };
        // Checks that stop square is valid
        let stop_square = match stop {
            Some(square) => square,
            None => return Err(MoveError::InvalidNotation),
        };

        // Checks that we aren't trying to move an empty square
        let mut piece = match self.board_state.get_piece_square(&start_square) {
            Some(piece) => piece,
            None => return Err(MoveError::EmptySquare),
        };

        // Checks that the piece being moved is the same as the color of who's turn it is
        if piece.is_white() != self.is_white_turn {
            return Err(MoveError::NotYourPiece);
        }

        // Checks that we aren't trying to capture own piece
        if self.board_state.is_same_color(&start_square, &stop_square) {
            return Err(MoveError::CantSelfHarm);
        }

        // If move passes the movement logic check
        if !self.movement_logic.check_move(&self.board_state, &start_square, &stop_square) {
            return Err(MoveError::IllegalMove);
        }

        // Check that we aren't exposing king
        if self.hangs_king(&self.board_state, &start_square, &stop_square, self.is_white_turn) {
            return Err(MoveError::HangsKing);
        }

        // FROM HERE ON
        // Since the move is valid by the previous checks we can check how it would effect the fifty move rule
        self.movement_logic.fifty_handler(&self.board_state, &start_square, &stop_square);

        // If the move a king-castling move
        if self.movement_logic.is_valid_castle(&self.board_state, &start_square, &stop_square) {
            todo!()
            // Moves the castle to the right positions
        }

        // If the move is a double pawn push, setting up en passant for next move
        else if self.movement_logic.is_valid_en_passant_setup(&self.board_state, &start_square, &stop_square) {
            // Set the en_passant square to the square behind where the move stops at
            let direction: i8 = if piece.is_white() { 1 } else { -1 };
            let passant_square = Square::new_square_from_index(start_square.file(), start_square.rank() + direction).unwrap();
            self.movement_logic.set_en_passant_square(Some(passant_square));
        }

        // If the move is a en_passant capture move
        else if self.movement_logic.is_valid_en_passant_capture(&self.board_state, &start_square, &stop_square) {
            // Removes the "captured pawn" since its not on stop_square
            let captured_square = Square::new_square_from_index(stop_square.file(), start_square.rank()).unwrap();
            self.board_state.set_piece_square(&captured_square, None);
            self.movement_logic.set_en_passant_square(None);
        }

        // If the move is a pawn promotion
        else if self.movement_logic.is_valid_promotion(&self.board_state, &start_square, &stop_square) {
            todo!()
            // Moves the piece and changes the type to what it needs to be
            // (will need to add API support here so that someone who makes the move can choose what piece it's promoted to)
        }

        // Since piece is allowed to move to stop square, and we took care of special cases, we make move as normal
        piece.has_moved_true();
        self.board_state.set_piece_square(&start_square, Some(piece));  // Set start_square to same piece but moved
        self.board_state.move_piece_square(&start_square, &stop_square);            // Actually moves the piece

        self.is_white_turn = !self.is_white_turn;

        Ok(())
    }

    // Find square with king of given color
    fn find_king(board: &Board, is_white: bool) -> Option<Square> {
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
    fn is_attacked(&self,board: &Board, attacked_square: &Square, is_white: bool) -> bool {
        for file in 0..8 {
            for rank in 0..8 {
                // The square we want to see if it has LOS to attacked_square
                let checking_square = Square::new_square_from_index(file, rank).unwrap();

                match board.get_piece_file_rank(file, rank) {
                    Some(piece) 
                    => if piece.is_white() == is_white {
                        continue; // Skip if same color
                    },
                    None => continue, // Skip if empty
                }
                if self.movement_logic.check_move(board, &checking_square, attacked_square) {
                    return true;
                }
            }
        }
        return  false;
    }

    // Combine two previous to find out if in check
    fn is_in_check(&self, board: &Board, is_white: bool) -> bool {
        match Self::find_king(board, is_white) {
            Some(king_square) =>
            return self.is_attacked(board, &king_square, is_white),
            None => return false,
        }
    }

    // Checks if move from square to square would expose king
    fn hangs_king(&self, board: &Board, start: &Square, stop: &Square, is_white: bool) -> bool {
        let mut test_board = board.clone();
        test_board.move_piece_square(start, stop);
        self.is_in_check(&test_board, is_white)
    }

    // Checks if there are any legal moves
    // If not then game over
    fn has_legal_moves(&self, board: &Board, is_white: bool) -> bool {
        // Can lowkirkenuaneliey copy is_attacked
        for file in 0..8 {
            for rank in 0..8 {
                // The square we want to see if it has LOS to attacked_square
                let checking_square = Square::new_square_from_index(file, rank).unwrap();

                match board.get_piece_file_rank(file, rank) {
                    Some(piece) 
                    => if piece.is_white() == !is_white {
                        continue; // Skip if not correct color
                    },
                    None => continue, // Skip if empty
                }
                for dest_file in 0..8 {
                    for dest_rank in 0..8 {
                        let destination_square = Square::new_square_from_index(dest_file, dest_rank).unwrap();
                        if self.movement_logic.check_move(board, &checking_square, &destination_square) {
                            if !self.hangs_king(board, &checking_square, &destination_square, is_white) {
                                return true;
                            }
                        }
                    }
                } // Holy nesting!!! I would make less awful if i had the time
            }
        }
        return  false;
    }

    // In check with no legal moves
    pub fn is_checkmate(&self, board: &Board, is_white: bool) -> bool {
        self.is_in_check(board, is_white) && !self.has_legal_moves(board, is_white)
    }

    // Not in check but no legal moves
    pub fn is_stalemate(&self, board: &Board, is_white: bool) -> bool {
        !self.is_in_check(board, is_white) && !self.has_legal_moves(board, is_white)
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
        assert!(!game.make_move_notation("e2", "d3").is_ok());
    }

    #[test]
    fn checkmate_detection() {
        // Try to see if checkmate is detected
        let board = Board::board_from_strings([
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        ".....PPP",
        "r......K",
        ]);
        let test_game = ChessGame::new_game(board, true);
        assert!(test_game.is_checkmate(&board, true));
        assert!(!test_game.is_stalemate(&board, true));
    }

    #[test]
    fn stalemate_detection() {
        // Try to see if stalemate is detected
        let board = Board::board_from_strings([
        "k.......",
        "..Q.....",
        "........",
        "........",
        "..K.....",
        "........",
        "........",
        "........",
        ]);
        let test_game = ChessGame::new_game(board, true);
        assert!(test_game.is_stalemate(&board, false));
        assert!(!test_game.is_checkmate(&board, false));
    }

    #[test]
    fn en_passant() {
        // Try to see if en_passant capture works
        let board = Board::board_from_strings([
        "........",
        "........",
        "........",
        "........",
        "...p....",
        "........",
        "....P...",
        "........",
        ]);
        let mut game = ChessGame::new_game(board, true);
        assert!(game.make_move_notation("e2", "e4").is_ok());
        assert!(game.make_move_notation("d4", "e3").is_ok());
        assert!(game.board().get_piece_square(&Square::new_square_from_notation_str("e3").unwrap()).is_some());
        assert!(game.board().get_piece_square(&Square::new_square_from_notation_str("e4").unwrap()).is_none());
        assert!(game.board().get_piece_square(&Square::new_square_from_notation_str("d4").unwrap()).is_none());
    }

    #[test]
    fn fifty_move_rule_tie() {
        // Moves knight back and forth 50 times to see if counts as tie
        let board = Board::board_from_strings([
        ".n......",
        "........",
        "........",
        "........",
        "........",
        "........",
        "........",
        ".N......",
        ]);
        let mut game = ChessGame::new_game(board, true);
        assert!(!game.in_tie());

        // Makes 49 moves aka 98 turns
        let mut forward_turn = true;
        for _ in 0..49 {
            let (white_from, white_to) = if forward_turn { ("b1", "c3") } else { ("c3", "b1") };
            let (black_from, black_to) = if forward_turn { ("b8", "c6") } else { ("c6", "b8") };

            assert!(game.make_move_notation(white_from, white_to).is_ok());
            assert!(game.make_move_notation(black_from, black_to).is_ok());

            forward_turn = !forward_turn;
        }
        // Checks that game is not in tie after 49 moves
        assert!(!game.in_tie());

        // Makes 50th move
        let (white_from, white_to) = if forward_turn { ("b1", "c3") } else { ("c3", "b1") };
        let (black_from, black_to) = if forward_turn { ("b8", "c6") } else { ("c6", "b8") };

        assert!(game.make_move_notation(white_from, white_to).is_ok());
        assert!(game.make_move_notation(black_from, black_to).is_ok());

        // Checks that game IS tie after 50 moves
        assert!(game.in_tie());
    }
}