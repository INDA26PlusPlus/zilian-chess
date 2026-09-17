// The different types it can be 
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

// Piece structure
#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    piece_type: PieceType,
    is_white:   bool,
    has_moved:  bool,
}

impl ChessPiece {

    // Construct a new chess piece
    pub fn new(piece_type: PieceType, is_white: bool, has_moved: bool) -> Option<Self> {       
        Some(Self {
            piece_type,
            is_white,
            has_moved,
        })
    }

    // Takes a letter and gives the appropriate piece type (could l'key split in two in the future)
    pub fn piece_from_letter(letter: char, has_moved: bool) -> Option<Self> {

        let is_white: bool = letter.is_ascii_uppercase();
        let piece_type: PieceType = match letter.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return None,
        };
        Self::new(piece_type, is_white, has_moved)
    }

    // Takes a ChessPiece struct option and gives an appropriate ASCII symbol.
    pub fn letter_from_piece(piece: Option<Self>) -> char {
        if piece.is_none() {
            return '.';
        }

        let letter = match piece.unwrap().piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };

        if piece.unwrap().is_white {
            return letter.to_ascii_uppercase();
        }
        return letter;
    }

    // Getter's
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub fn is_white(&self) -> bool {
        self.is_white
    }
    pub fn has_moved(&self) -> bool {
        self.has_moved
    }
    
    // Setter's
    pub fn has_moved_true(&mut self) {
        self.has_moved = true;
    }
 
}