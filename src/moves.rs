/*
moves.rs is responsible for handling all the movement checks,
This is basically the brain of the library, when it's determined if
a move is valid or not.

Only issue is that it's built using like only bools which was nice and
easy when making the first pieces but I now have to implement special moves
and castling and en passant require more info than just a yes/no
*/
use crate::board::{Board, Square};
use crate::pieces::PieceType;

// Using these flags we'll hopefully be able to make some special moves happen.
// I'm high-key WAY to tired to do this on the bus right now

pub enum SpecialMove {
    Normal,
    Castling {rook_start: Square, rook_stop: Square},


    Promotion {promotion_square: Square},

    // If a double pawn move has been made previously
    // Square is the skipped pawn square (behind the pawn that moved)
    EnPassant {en_passant_square: Square}
}
pub struct Move {
    // Flags depending on if the move was "special"
    // Aka, castling, en_passant, promotion and needs extra handling
    flag: SpecialMove,
    // Counts moves since a pawn or capture move (50 move rule)
    since_pawn_capture: u8
}

impl Move {

    pub fn new_move () -> Self {
        Self {
            flag: SpecialMove::Normal,
            since_pawn_capture: 0
        }
    }

    // Increments the 50 move rule
    pub fn fifty_increment(&mut self) {
        self.since_pawn_capture += 1;
    }

    // Resets 50 move rule
    pub fn fifty_reset(&mut self) {
        self.since_pawn_capture = 0;
    }

    pub fn fifty_handler(&mut self, board: &Board, start: &Square, stop: &Square) {
        // Is it capture -> Reset
        if board.would_be_capture(start, stop) {
            self.fifty_reset();
        }
        // Is it pawn moving -> Reset
        else if board.get_piece_square(start).unwrap().piece_type() == PieceType::Pawn {
            self.fifty_reset();
        }
        // None of the above -> Reset
        else {
            self.fifty_increment();
        }
    }

    // Runs different movement checks depending on starting piece.
    pub fn check_move(&self, board: &Board, start: &Square, stop: &Square) -> bool {

        // Gets piece at start and check
        let piece = match board.get_piece_square(start) {
            Some(piece) => piece,
            None => return false // Starting square empty,
        };

        match piece.piece_type() {
            PieceType::Pawn     => self.pawn_move_check    (board, start, stop),
            PieceType::Knight   => self.knight_move_check  (board, start, stop),
            PieceType::Bishop   => self.bishop_move_check  (board, start, stop),
            PieceType::Rook     => self.rook_move_check    (board, start, stop),
            PieceType::Queen    => self.queen_move_check   (board, start, stop),
            PieceType::King     => self.king_move_check    (board, start, stop),
        }
    }

    // Sets flag
    pub fn set_flag(&mut self, flag: SpecialMove) {
        self.flag = flag;
    }

    // Gets flag 
    pub fn get_flag(&self) -> &SpecialMove {
        &self.flag
    }

    pub fn en_passant_flag(passant_square: Square) -> SpecialMove {
        SpecialMove::EnPassant { en_passant_square: passant_square }
    }

    // Move diagonal in all directions, steps until collision.
    fn bishop_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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
    fn rook_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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
    fn queen_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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
    fn knight_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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

        // Checks that move is valid and if capture, it's not same piece.
        return move_match && !board.is_same_color(start, stop);
    }

    // Move any direction. 1 step.
    // If enemy has LOS, mark as checked
    // If friendly rook with has_moved = false and king has_moved = false
        // Allow castle by taking 2 steps to the rook and moving the rook next to the king
    fn king_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
        const KING_MOVES: [(i8, i8); 8] = [
            (1,1),  (1,-1),  (1,0), (-1,0),
            (-1,1), (-1,-1), (0,1), (0,-1),
        ];

        let mut move_match = false;

        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        for (rank_move, file_move) in KING_MOVES {
            if rank_move == rank_change && file_move == file_change {
                move_match = true;
            }
        }

        // Checks that move is valid and if capture, it's not same piece.
        if move_match {
            return !board.is_same_color(start, stop);
        }

        ////// BEGINNING OF A VERY NOT GOOD CASTLING SCRIPT
        if rank_change != 0 || (file_change != 2 && file_change != -2) {
            return false;
        }
        // Past here we're checking if its a valid castling move

        // Find the king
        let king = match board.get_piece_square(start) {
            Some(piece) => piece,
            None => return  false,
        };

        // Has king moved
        if king.has_moved() {
            return false;
        }

        // Is king in check
        if self.square_attacked(board, start, king.is_white()) {
            return false;
        }

        // Iterate and check that all the squares to the rook are empty
        // On square 1 and 2 of iteration check if that spot is in check
        // On reaching (or not reaching rook) se if the rook exists or has_moved
        let step: i8;
        if file_change > 0 {
            step = 1
        }
        else {
            step = -1
        }

        let mut _file = start.file() + step;


        // If no check and clear path to rook, nobody has_moved


    return false;

    }

    // Scans the board for an enemy piece that can reach `square`, used to
    // check the king isn't in, through, or landing in check while castling.
    fn square_attacked(&self, board: &Board, square: &Square, is_white: bool) -> bool {
        for file in 0..8 {
            for rank in 0..8 {
                let attacker_square = match Square::new_square_from_index(file, rank) {
                    Some(square) => square,
                    None => continue,
                };

                match board.get_piece_square(&attacker_square) {
                    Some(piece) if piece.is_white() != is_white => {
                        if self.check_move(board, &attacker_square, square) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        false
    }

    // Since a Pawn doesn't "step" like a Rook, Bishop or Queen
    // We can take part of the Knight logic and look for if the
    // Change between the start and stop square is possible
    pub fn pawn_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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
            if rank_move == 2 && file_move == 0 && (board.get_piece_square(stop).is_none() && board.get_piece_file_rank(stop.file(), stop.rank() - direction).is_none()) && !board.get_piece_square(start).unwrap().has_moved() {
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
            if 0 > rank || rank >= 8 || 0 > file || file >= 8 {
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
            let start = Square::new_square_from_notation_str("c3").unwrap();
            let stop= Square::new_square_from_notation_str("c4").unwrap();
            // Must be true or cargo test will fail
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e2").unwrap();
            let stop= Square::new_square_from_notation_str("e3").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e2").unwrap();
            let stop= Square::new_square_from_notation_str("e3").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e2").unwrap();
            let stop= Square::new_square_from_notation_str("e4").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e2").unwrap();

            // Sets the pawn "has_moved" to true
            board.set_piece_square(&start, ChessPiece::new(Pawn, true, true));

            let stop= Square::new_square_from_notation_str("e4").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e2").unwrap();
            let stop= Square::new_square_from_notation_str("e4").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e5").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e5").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e5").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e7").unwrap();
            let stop= Square::new_square_from_notation_str("e5").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("e7").unwrap();
            let stop= Square::new_square_from_notation_str("f6").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("f4").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("h8").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("b2").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("c5").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d8").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("a8").unwrap();
            let stop= Square::new_square_from_notation_str("g8").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("g3").unwrap();
            let stop= Square::new_square_from_notation_str("f4").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d2").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d6").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("b7").unwrap();
            let stop= Square::new_square_from_notation_str("d5").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
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
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d2").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
        }
    }

    mod king_test {
        use super::*;

        #[test]
        fn king_valid_move() {
            // Try a valid king move on empty board
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...K....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d5").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
        }

        #[test]
        fn king_enemy_capture() {
            // Try valid white on black king capture 
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "...Kp...",
            "........",
            "........",
            "........",
            "........",
            ]);
            let start = Square::new_square_from_notation_str("d5").unwrap();
            let stop= Square::new_square_from_notation_str("e5").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
        }


        #[test]
        fn king_invalid_move() {
            // Try moving queen in invalid way
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...k....",
            "........",
            "........",
            "........",
            ]);
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
        }

        #[test]
        fn king_team_capture() {
            // Try white on white king capture
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "...K....",
            "...P....",
            "........",
            "........",
            ]);
            let start = Square::new_square_from_notation_str("d4").unwrap();
            let stop= Square::new_square_from_notation_str("d3").unwrap();
            assert!(!Move::check_move(&Move::new_move(), &board, &start, &stop));
        }
    }
}