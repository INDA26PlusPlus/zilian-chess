use crate::pieces::ChessPiece;


// A nested array (Mailbox approach) that can either be empty or contain a piece.
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

    // Sets a piece on a given square.
    pub fn set_piece_square(&mut self, square: Square, chess_piece: Option<ChessPiece>) {
        self.squares[square.file as usize][square.rank as usize] = chess_piece;
    }

    // Gets the piece from a given square.
    pub fn get_piece_square(&self, square: &Square) -> Option<ChessPiece> {
        self.squares[square.file as usize][square.rank as usize]
    }

}

// Struct for the file and rank of a square on the board
pub struct Square {
    file: u8,
    rank: u8
}

impl Square {

    // Make new square from two values.
    pub fn new_square_from_index(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Self {file, rank})
        }
        else {
            None
        }
    }

    // Takes a two parts of chess notation, say a1 and converts it into a square of index's 0, 0
    pub fn new_square_from_notation(file: char, rank: u8) -> Option<Self> {
        // Change file from letter to index
        let converted_file: u8 = match file {
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
        let converted_rank: u8;
        if rank >= 8 || rank <= 0 {
            return None;
        }
        else {
            converted_rank = rank - 1;
        }
        
        return Self::new_square_from_index(converted_file, converted_rank);
    }

    // Getter's
    pub fn file(&self) -> u8 {
        self.file
    }
    pub fn rank(&self) -> u8 {
        self.rank
    }

}