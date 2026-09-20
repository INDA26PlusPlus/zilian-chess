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

pub struct Move {
    // If a double pawn move has been made previously
    // Square is the skipped pawn square (behind the pawn that moved)
    en_passant_square: Option<Square>,
    // Counts moves since a pawn or capture move (50 move rule)
    since_pawn_capture: u8
}

impl Move {

    pub fn new_move () -> Self {
        Self {
            en_passant_square: None,
            since_pawn_capture: 0
        }
    }

    // Increments the 50 move rule
    fn fifty_increment(&mut self) {
        self.since_pawn_capture += 1;
    }

    // Resets 50 move rule
    fn fifty_reset(&mut self) {
        self.since_pawn_capture = 0;
    }

    pub fn fifty_handler(&mut self, board: &Board, start: &Square, stop: &Square) {
        // Is it capture -> Reset
        if board.would_be_capture(start, stop) {
            self.fifty_reset();
        }
        // Is it pawn moving -> Reset (also takes care of en passant)
        else if board.get_piece_square(start).unwrap().piece_type() == PieceType::Pawn {
            self.fifty_reset();
        }
        // None of the above -> Reset
        else {
            self.fifty_increment();
        }
    }

    pub fn get_fifty_increment(&self) -> u8 {
        return self.since_pawn_capture;
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

    pub fn set_en_passant_square(&mut self, square: Option<Square>) {
        self.en_passant_square = square;
    }

    // Checks if the move would be a valid castling move
    pub fn is_valid_castle(&self, board: &Board, start: &Square, stop: &Square) -> bool {

        // Checks that piece exists
        let king = match board.get_piece_square(start) {
            Some(piece) => piece,
            None => return false,
        };

        // Checks that piece is king
        if king.piece_type() != PieceType::King || king.has_moved() {
            return false;
        }

        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        // Checks that move is trying to take 2 steps to the side
        if rank_change != 0 || (file_change != 2 && file_change != -2) {
            return false;
        }

        // Cant if in check
        if self.square_attacked(board, start, king.is_white()) {
            return false;
        }

        // Sets castling direction
        let step: i8 = if file_change > 0 { 1 } else { -1 };

        // Checks the rook is eligible, the path to it is clear, and the king
        // doesn't pass through or land on an attacked square.
        self.castling_directional_check(board, start, step)
    }

    // Checks if the move is a double pawn push, enabling en passant next move
    pub fn is_valid_en_passant_setup(&self, board: &Board, start: &Square, stop: &Square) -> bool {
        let piece = board.get_piece_square(start).unwrap();

        if piece.piece_type() != PieceType::Pawn {
            return false;
        }

        let direction: i8 = if piece.is_white() {1} else {-1};
        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        return rank_change == 2 * direction && file_change == 0;
    }

    // Checks if the move captures a pawn via en passant
    pub fn is_valid_en_passant_capture(&self, board: &Board, start: &Square, stop: &Square) -> bool {
        let piece = board.get_piece_square(start).unwrap();

        if piece.piece_type() != PieceType::Pawn {
            return false;
        }

        let direction: i8 = if piece.is_white() {1} else {-1};
        let rank_change = stop.rank() - start.rank();
        let file_change = stop.file() - start.file();

        return rank_change == direction && (file_change == 1 || file_change == -1) && self.en_passant_square.as_ref() == Some(stop);
    }

    // Checks if move would be a promotion
    pub fn is_valid_promotion(&self, board: &Board, start: &Square, stop: &Square) -> bool {
        let piece = match board.get_piece_square(start) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.piece_type() != PieceType::Pawn {
            return false;
        }

        // Checks that its going to the back-rank (1 if black 8 if white)
        let back_rank = if piece.is_white() { 7 } else { 0 };
        if stop.rank() != back_rank {
            return false;
        }

        return self.pawn_move_check(board, start, stop);
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

        // Not a valid normal king move so check if its a castling move
        return self.is_valid_castle(board, start, stop);
    }

    // Since a Pawn doesn't "step" like a Rook, Bishop or Queen
    // We can take part of the Knight logic and look for if the
    // Change between the start and stop square is possible
    fn pawn_move_check(&self, board: &Board, start: &Square, stop: &Square) -> bool {
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
            if self.is_valid_en_passant_setup(board, start, stop)
            && (board.get_piece_square(stop).is_none()
            && board.get_piece_file_rank(stop.file(), stop.rank() - direction).is_none())
            && !board.get_piece_square(start).unwrap().has_moved() {
                move_match = true;
            }

            // Capture (given that it's a normal capture)
            if rank_move == 1 && (file_move == 1 || file_move == -1) && !board.get_piece_square(stop).is_none() {
                move_match = true;
            }

            // Capture (en passant)
            if self.is_valid_en_passant_capture(board, start, stop) {
                move_match = true;
            }

        }
        return move_match && !board.is_same_color(start, stop);
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

            // Go to new direction cause outside boundary
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

    // Checks castling is ok based of direction
    fn castling_directional_check(&self, board: &Board, start: &Square, file_change: i8) -> bool {

        // Determine color
        let king_is_white = board.get_piece_square(start).unwrap().is_white();

        // Determines the rook position based of direction (file_change -1 or 1)
        let rook_file = if file_change > 0 { 7 } else { 0 };
        let rook_square = Square::new_square_from_index(rook_file, start.rank()).unwrap();

        // Checks that rook exists on the a or h file
        let rook = match board.get_piece_square(&rook_square) {
            Some(piece) => piece,
            None => return false,
        };
        // Rook hasn't moved and is same color
        if rook.piece_type() != PieceType::Rook || rook.has_moved() || rook.is_white() != king_is_white {
            return false;
        }

        // Steps towards the rook checking that path is clear
        let rank = start.rank();
        let mut file = start.file();
        // Makes sure that the first 2 steps aren't in check
        let mut steps = 0;

        loop {
            // Takes step
            file = file + file_change;

            // Reached rook square -> Path is all clear
            if file == rook_square.file() {
                return true;
            }

            // Anything in the path -> False
            if board.get_piece_file_rank(file, rank).is_some() {
                return false;
            }

            // The first two steps must be safe
            steps += 1;
            if steps <= 2 {
                let square = Square::new_square_from_index(file, rank).unwrap();
                if self.square_attacked(board, &square, king_is_white) {
                    return false;
                }
            }
        }
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
            // Try a black pawn single, going down the board instead of up
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
            let stop= Square::new_square_from_notation_str("e6").unwrap();
            assert!(Move::check_move(&Move::new_move(), &board, &start, &stop));
        }

                #[test]
        fn pawn_white_single() {
            // Try a black pawn single, going down the board instead of up
            let board = Board::board_from_strings([
            "........",
            "........",
            "........",
            "........",
            "........",
            "P.......",
            "........",
            "........",
            ]);
            let start = Square::new_square_from_notation_str("a3").unwrap();
            let stop= Square::new_square_from_notation_str("a4").unwrap();
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