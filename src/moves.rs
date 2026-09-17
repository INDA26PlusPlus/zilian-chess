use crate::board::{Board, Square};

// What is a move?
// One place to another
// What properties can a move have?
//      Legal/Illegal, Normal/ Special (en passant ect)
//      Direction? Like white and black go opposite (7-rank to swap)
pub struct Move {
    pub start: Square,
    pub stop: Square,
}

impl Move{

    // Will later turn into the core of the program
    // All checks to see if a given move is legal will happen from here
    // For now we'll just say it's always legal so we can get ANY move to happen
    pub fn check_move(_board: &Board, _check_start: &Square, _check_stop: &Square) -> bool {
        return true;
    }
}