use crate::pieces::ChessPiece;


// A nested array (Mailbox approach) that can either be empty or contain a piece.
#[derive(Debug, Copy, Clone)]
pub struct Board {
    squares: [[Option<ChessPiece>; 8]; 8]
}

impl Board {

    // Makes a fully empty board, good for testing eventually.
    pub fn new_empty_board() -> Self {
        Self {
            squares: [[None; 8]; 8],
        }
    }

    pub fn board_from_strings(string_array: [&str; 8]) -> Self {

        let mut board = Self::new_empty_board();

        // For each rank (1,2,3,...) 
        for rank in 0..8 { // Needs to be flipped
            // For each file (a,b,c,...)
            for file in 0..8 {
                let letter = string_array[rank].as_bytes()[file] as char;
                // Here we flip the rank cause the boards 0,0 is a1 but in the string 0,0 is a8
                board.squares[file][7 - rank] = ChessPiece::piece_from_letter(letter, false);
            }
        }

        return board;
    }

    pub fn new_starting_board() -> Self {
        Board::board_from_strings([
            "rnbqkbnr",
            "pppppppp",
            "........",
            "........",
            "........",
            "........",
            "PPPPPPPP",
            "RNBKQBNR",
        ])
    }

    // Sets a piece on a given square.
    pub fn set_piece_square(&mut self, square: &Square, chess_piece: Option<ChessPiece>) {
        self.squares[square.file as usize][square.rank as usize] = chess_piece;
    }

    // Gets the piece from a given square.
    pub fn get_piece_square(&self, square: &Square) -> Option<ChessPiece> {
        self.squares[square.file as usize][square.rank as usize]
    }
    // Gets piece from given index
    pub fn get_piece_file_rank(&self, file: i8, rank: i8) -> Option<ChessPiece> {
        return self.get_piece_square(&Square::new_square_from_index(file, rank).unwrap());
    }

    // Moves (Really just copies then removes) whats on start to stop.
    pub fn move_piece_square(&mut self, start: &Square, stop: &Square) {
        let piece = self.get_piece_square(start);
        self.set_piece_square(start, None);
        self.set_piece_square(stop, piece);
    }

    // Checks if two squares have same color
    pub fn is_same_color(&self, start: &Square, stop: &Square) -> bool {
        let start_piece = match self.get_piece_square(start) {
            Some(piece) => piece,
            None => return false,
        };

        let stop_piece = match self.get_piece_square(stop) {
            Some(piece) => piece,
            None => return false,
        };

        start_piece.is_white() == stop_piece.is_white()
    }

}

// Struct for the file and rank of a square on the board
pub struct Square {
    file: i8,
    rank: i8
}

impl Square {

    // Make new square from two values.
    pub fn new_square_from_index(file: i8, rank: i8) -> Option<Self> {
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            Some(Self {file, rank})
        }
        else {
            None
        }
    }

    // Takes a two parts of chess notation, say a1 and converts it into a square of index's 0, 0
    pub fn new_square_from_notation(file: char, rank: i8) -> Option<Self> {
        // Change file from letter to index
        let converted_file: i8 = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None
        };

        // Change the rank from number to index
        let converted_rank: i8;
        if rank > 8 || rank <= 0 {
            return None;
        }
        else {
            converted_rank = rank - 1;
        }
        
        return Self::new_square_from_index(converted_file, converted_rank);
    }

    // Turns a string of chess notation "a1" into a Square.
    pub fn square_from_notation_str(notation_string: &str) -> Option<Self> {
        // Not Good
        if notation_string.len() != 2 {
            return None;
        }

        let rank: i8 = notation_string.chars().nth(1).unwrap().to_digit(10).unwrap() as i8;
        let file: char = notation_string.as_bytes()[0] as char;

        Self::new_square_from_notation(file, rank)
    }

    // Getter's
    pub fn file(&self) -> i8 {
        self.file
    }
    pub fn rank(&self) -> i8 {
        self.rank
    }

}