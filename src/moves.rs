// Protip Alt+z to read the comments, they are quite long sometimes and text wrapping makes life easier

use std::io::pipe;

use crate::board::{Board, Square};
use crate::pieces::PieceType::{self, Bishop};

// What is a move?
// One place to another
// What properties can a move have?
//      Legal/Illegal, Normal/ Special (en passant ect)
//      Direction? Like white and black go opposite (7-rank to swap)
pub struct Move {
    pub start: Square,
    pub stop: Square,
}

impl Move {

    // Runs different movement checks depending on starting piece.
    pub fn check_move(board: &Board, start: &Square, stop: &Square) -> bool {

        // Gets piece at start and check
        let piece = match board.get_piece_square(start) {
            Some(piece) => piece,
            None => return false // Starting square empty,
        };

        match piece.piece_type() {
            PieceType::Pawn     => Self::pawn_move_check    (board, start, stop),
            PieceType::Knight   => Self::knight_move_check  (board, start, stop),
            PieceType::Bishop   => Self::bishop_move_check  (board, start, stop),
            PieceType::Rook     => Self::rook_move_check    (board, start, stop),
            PieceType::Queen    => Self::queen_move_check   (board, start, stop),
            PieceType::King     => Self::king_move_check    (board, start, stop),
        }
    }

    // Move diagonal in all directions, steps until collision.
    fn bishop_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        const BISHOP_MOVE_DIRECTION: [(i8, i8); 4] = [
            (1,1),  (1,-1),
            (-1,1), (-1,-1),
        ];

        // Goes through each direction using stepping method.
        for direction in BISHOP_MOVE_DIRECTION {
            if Self::direction_stepping(board, start, stop, direction) {
                return true;
            }
        }
        return false;
    }

    // Move cardinally any direction, endless steps.
    fn rook_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        const ROOK_MOVE_DIRECTION: [(i8, i8); 4] = [
            (1,0),  (-1,0),
            (0,1), (0,-1),
        ];

        // Goes through each direction using stepping method.
        for direction in ROOK_MOVE_DIRECTION {
            if Self::direction_stepping(board, start, stop, direction) {
                return true;
            }
        }
        return false;
    }

    // Combination of rook and bishop
    fn queen_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        const QUEEN_MOVE_DIRECTION: [(i8, i8); 8] = [
            (1,1),  (1,-1),  (1,0), (-1,0),
            (-1,1), (-1,-1), (0,1), (0,-1),
        ];

        // Goes through each direction using stepping method.
        for direction in QUEEN_MOVE_DIRECTION {
            if Self::direction_stepping(board, start, stop, direction) {
                return true;
            }
        }
        return false;
    }

    // Move 2+1 in any direction. 1 step
    fn knight_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        const KNIGHT_MOVES: [(i8, i8); 8] = [
            (2,1),  (2,-1),  (1,2),   (1,-2), 
            (-1,2), (-1,-2), (-2, 1), (-2, -1),
        ];

        let mut move_match = false;

        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        for (rank_move, file_move) in KNIGHT_MOVES {
            if rank_move == rank_change && file_move == file_change {
                move_match = true;
            }
        }

        move_match && !board.is_same_color(start, stop)
    }

    // Move any direction. 1 step.
    // If enemy has LOS, mark as checked
    // If friendly rook with has_moved = false and king has_moved = false
        // Allow castle by taking 2 steps to the rook and moving the rook next to the king
    fn king_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    }

    // If has_moved = true, then up 1 in any direction
    // Only allow diagonal up when capture = allowed
    // If piece on either side is enemy pawn with flag, also allow diagonal up
    // If has_moved = false, also allow 2 up center
        // Flag piece
    fn pawn_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        true
    }

    // Takes a single direction and steps in that direction repeatedly until boundary or another piece is hit.
    fn direction_stepping(board: &Board, start: &Square, stop: &Square, direction: (i8, i8)) -> bool {

        // Starting piece (important later for )
        let starting_piece = board.get_piece_square(start).unwrap();
        let (rank_change, file_change) = direction; // Extract value from tuple

        let mut rank = start.rank();
        let mut file = start.file();

        loop {
            // Takes step
            rank = rank + rank_change;
            file = file + file_change;

            // Go to new direction outside boundary
            if (0 > rank || rank >= 8 || 0 > file || file >= 8) {
                break;
            }

            // Given that new step is in bounds, what piece is it?
            let stepped_piece = board.get_piece_file_rank(file, rank);

            match stepped_piece {
                Some(piece) => {
                    // Hit a piece
                    if rank == stop.rank() && file == stop.file() {
                        // Allow capturing if colors are different.
                        return starting_piece.is_white() != piece.is_white();
                    }
                    break; // Can't go further
                }

                None => {
                    // Hit nothing
                    if rank == stop.rank() && file == stop.file() {
                        return true;
                    }
                    // Keep 'er goin'
                }
            }
        }

        return false;
    }

    // If nothing blocks the final destination then valid
    // For rook, bishop, queen
    // Also first move pawn
    // Also castling king
    fn move_obstruction(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    } 
}


// Testing block for all tests
// Testing is grouped by piece for easier readability
// and so i can collapse these later
#[cfg(test)]
mod tests {
    use super::*;

    mod general_tests {

        use super::*;

        #[test]
        fn move_from_empty() {
            // Try moving FROM a square thats empty
            let board = Board::board_from_strings([
            "rnbqkbnr",
            "pppppppp",
            "........",
            "........",
            "........",
            "........",
            "PPPPPPPP",
            "RNBKQBNR",
            ]);
            let start = Square::square_from_notation_str("c3").unwrap();
            let stop= Square::square_from_notation_str("c4").unwrap();
            // Must be true or cargo test will fail
            assert!(!Move::check_move(&board, &start, &stop));
        }
    }

    mod knight_tests {
        use super::*;

        #[test]
        fn knight_valid_move() {
            // Try a valid knight move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...N....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e6").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn knight_enemy_capture() {
            // Try valid white on black knight capture 
            let board = Board::board_from_strings([
            "........",
            "........",
            "....p...",
            "........",
            "...N....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e6").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }


        #[test]
        fn knight_invalid_move() {
            // Try moving knight in invalid way
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...N....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("f4").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn knight_team_capture() {
            // Try white on white knight capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "....P...",
            "........",
            "...N....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

    }



        mod bishop_test {
        use super::*;

        #[test]
        fn bishop_valid_move() {
            !todo!()
        }
    }
}