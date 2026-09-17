// Protip Alt+z to read the comments, they are quite long sometimes and text wrapping makes life easier

use crate::board::{Board, Square};
use crate::pieces::PieceType;

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

    // Looks at what's on `start` and branches to the matching piece check.
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

    // Move diagonal any direction, endless steps.
    fn bishop_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    }

    // Move cardinally any direction, endless steps.
    fn rook_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    }

    // Combination of rook and bishop
    fn queen_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        false
    }

    // Move 2+1 in any direction. 1 step
    fn knight_move_check(board: &Board, start: &Square, stop: &Square) -> bool {
        // I'll start with the knight since it has the most basic logic, can jump over pieces and has no special rules or flags.

        // Say knight on b1 wants to move to a3, it will check the different movement vectors of the knight, going from (2,1), (2,-1), (1,2), (1,-2), (-1,2), (-1,-2), (-2, 1), (-2, -1) where the first value is the vertical (rank) change and the second is the horizontal (file)
        // We go through each of these, if we are about to exit the bounds of the board we return false, if none match we return false, otherwise true

        // We also need to check that the destination square (stop) square doesn't contain a piece of the same color, we then refer to the "destination_color" check method

        // If both of these are true, meaning we won't capture our own piece, we wont go out of bounds the move is valid

        // NOTE: We can't let a move expose our king, but this would mean checking if ANY of the enemy pieces can see our king, since we haven't made the movement logic for the rest of the pieces, we wont add this check until later

        // Declare the ways a knight can move
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
        fn knight_self_capture() {
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
}