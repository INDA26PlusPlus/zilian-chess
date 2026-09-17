// Testing file to visually interpret the code
// Easier debugging and lets me use the API
// so i can see what I need to do to improve
// it from a user point-of-view.

use zilian_chess::board::{Board, Square};
use zilian_chess::pieces::ChessPiece;


fn main () {
    // Makes an empty board and places two pieces on it.
    let mut board = Board::new_empty_board();

    let square = Square::new_square_from_index(3, 2).unwrap();
    let chess_piece = ChessPiece::piece_from_letter('n', false);
    board.set_piece_square(square, chess_piece);

    let square = Square::new_square_from_notation('a', 5).unwrap();
    let chess_piece = ChessPiece::piece_from_letter('P', false);
    board.set_piece_square(square, chess_piece);

    println!("------------------");
    display_board(board);
    println!("------------------");
}

fn display_board (board: Board) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = Square::new_square_from_index(file, rank).unwrap();
            let piece = board.get_piece_square(&square);
            print!("{} ", ChessPiece::letter_from_piece(piece));
        }
        println!();
    }
    println!("  a b c d e f g h");
}
