// Protip Alt+z to read the comments, they are quite long sometimes and text wrapping makes life easier
// Future me here ^ ??? no code is way harder to read

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

        return move_match && !board.is_same_color(start, stop);
    }

    // Move any direction. 1 step.
    // If enemy has LOS, mark as checked
    // If friendly rook with has_moved = false and king has_moved = false
        // Allow castle by taking 2 steps to the rook and moving the rook next to the king
    fn king_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    }

    // Since a Pawn doesn't "step" like a Rook, Bishop or Queen
    // We can take part of the Knight logic and look for if the
    // Change between the start and stop square is possible
    fn pawn_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        const PAWN_MOVES: [(i8, i8); 4] = [
            (1,0),  (1,-1),
            (1,1), (2,0)
        ];

        let mut move_match = false;

        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        // Sets direction depending on if white or black (black moves in -rank)
        let direction: i8 = if board.get_piece_square(start).unwrap().is_white() {1} else {-1};

        for (rank_move, file_move) in PAWN_MOVES {

            // If the square isn't reachable by this move, skip
            if rank_move * direction != rank_change || file_move != file_change {
                continue;
            }

            // Given that the move is achievable
            if rank_move == 1 && file_move == 0 && board.get_piece_square(stop).is_none() {
                move_match = true;
            }

            // Checks if square empty and square before it as well if has_moved.
            if rank_move == 2 && file_move == 0 && (board.get_piece_square(stop).is_none() && board.get_piece_file_rank(stop.file(), stop.rank() - 1).is_none()) && !board.get_piece_square(start).unwrap().has_moved() {
                move_match = true;
                // EN PASSANT FLAG FOR NEXT MOVE
            }
            
            if rank_move == 1 && (file_move == 1 || file_move == -1) && !board.get_piece_square(stop).is_none() {
                move_match = true;
            }
        }
        return move_match && !board.is_same_color(start, stop);
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

    mod pawn_tests {
        // MI BOMBACLART
        use super::*;
        use crate::pieces::{ChessPiece, PieceType::Pawn};

        #[test]
        fn pawn_valid_forward_one() {
            // Try a valid pawn move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "....P...",
            "........",
            ]);
            let start = Square::square_from_notation_str("e2").unwrap();
            let stop= Square::square_from_notation_str("e3").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_forward_one_blocked() {
            // Try moving pawn forward into an occupied square, not even a capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "....p...",
            "....P...",
            "........",
            ]);
            let start = Square::square_from_notation_str("e2").unwrap();
            let stop= Square::square_from_notation_str("e3").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_valid_forward_two() {
            // Try a valid double step on the first move
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "....P...",
            "........",
            ]);
            let start = Square::square_from_notation_str("e2").unwrap();
            let stop= Square::square_from_notation_str("e4").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_forward_two_has_moved() {
            // Try double step after the pawn has already moved once
            let mut board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "....P...",
            "........",
            ]);
            let start = Square::square_from_notation_str("e2").unwrap();

            // Sets the pawn "has_moved" to true
            board.set_piece_square(&start, ChessPiece::new(Pawn, true, true));

            let stop= Square::square_from_notation_str("e4").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_forward_two_blocked() {
            // Try double step when piece in between
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "....p...",
            "....P...",
            "........",
            ]);
            let start = Square::square_from_notation_str("e2").unwrap();
            let stop= Square::square_from_notation_str("e4").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_enemy_capture() {
            // Try valid white on black pawn capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "....p...",
            "...P....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e5").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_diagonal_into_empty_invalid() {
            // Try moving pawn diagonally with nothing to capture (no en passant)
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...P....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e5").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_team_capture() {
            // Try white on white pawn capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "....P...",
            "...P....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e5").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_black_direction() {
            // Try a black pawn double step, going down the board instead of up
            let board = Board::board_from_strings([
            "........",
            "....p...",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("e7").unwrap();
            let stop= Square::square_from_notation_str("e5").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn pawn_black_enemy_capture() {
            // Try a black pawn double step, going down the board instead of up
            let board = Board::board_from_strings([
            "........",
            "....p...",
            ".....P..",
            "........",
            "........",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("e7").unwrap();
            let stop= Square::square_from_notation_str("f6").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
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
            // Try a valid bishop move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...B....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("h8").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn bishop_enemy_capture() {
            // Try valid white on black bishop capture 
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...B....",
            "........",
            ".p......",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("b2").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }


        #[test]
        fn bishop_invalid_move() {
            // Try moving bishop in invalid way
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...B....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn bishop_team_capture() {
            // Try white on white bishop capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "..P.....",
            "...B....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("c5").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }
    }

     mod rook_test {
        use super::*;

        #[test]
        fn rook_valid_move() {
            // Try a valid rook move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...R....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("d8").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn rook_enemy_capture() {
            // Try valid black on white rook capture 
            let board = Board::board_from_strings([
            "r.....P.",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("a8").unwrap();
            let stop= Square::square_from_notation_str("g8").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }


        #[test]
        fn rook_invalid_move() {
            // Try moving rook in invalid way
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "......R.",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("g3").unwrap();
            let stop= Square::square_from_notation_str("f4").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn rook_team_capture() {
            // Try black on black rook capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...R....",
            "........",
            "...P....",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("d2").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }
    }

    mod queen_test {
        use super::*;

        #[test]
        fn queen_valid_move() {
            // Try a valid queen move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...q....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("d6").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn queen_enemy_capture() {
            // Try valid black on white queen capture 
            let board = Board::board_from_strings([
            "........",
            ".q......",
            "........",
            "...P....",
            "........",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("b7").unwrap();
            let stop= Square::square_from_notation_str("d5").unwrap();
            assert!(Move::check_move(&board, &start, &stop));
        }


        #[test]
        fn queen_invalid_move() {
            // Try moving queen in invalid way
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...Q....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }

        #[test]
        fn queen_team_capture() {
            // Try white on white queen capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...Q....",
            "........",
            "...P....",
            "........",
            ]);
            let start = Square::square_from_notation_str("d4").unwrap();
            let stop= Square::square_from_notation_str("d2").unwrap();
            assert!(!Move::check_move(&board, &start, &stop));
        }
    }
}