// Testing file to visually interpret the code
// Easier debugging and lets me use the API
// so i can see what I need to do to improve
// it from a user point-of-view.

use zilian_chess::board::{Board, Square};
use zilian_chess::pieces::ChessPiece;
use zilian_chess::game::ChessGame;
use std::io;


fn main () {

    // Makes a starting position board now as part of a new "game"
    let mut game: ChessGame = ChessGame::new_game(Board::new_starting_board(), true);

    let mut exit: bool = false;
    while !exit {
        println!("------------------");
         display_board(game.board());
        println!("---{}---", game.turn_text());

        let mut input_start = String::new();
        let mut input_stop =  String::new();

        println!("Enter Starting Position");
        io::stdin()
            .read_line(&mut input_start)
            .expect("Failed to read line");
        println!("Enter Stopping Position");
        io::stdin()
            .read_line(&mut input_stop)
            .expect("Failed to read line");

        // Makes move and matches output with either OK (move successful)
        // Or Err (error occurred)
        match game.make_move(
            &input_start.trim().to_lowercase(),
            &input_stop.trim().to_lowercase()
            ) {
                Ok(()) => {},
                Err(message) => println!("{}", message)
            }
        }

    /*
    The move should happen as part of a game
    ChessGame should check with Moves to see if the move is valid given th board an piece 
    (also things like flags for en passant ect)
    The move is then made and can be shown to the user
    )
    */
}

// Display function that goes through rank and file and attaches a grid
fn display_board (board: &Board) { // Displays copies of the board since we're can't let ownership leave ChessGame
    for rank in (0..8).rev() {
        print!("{} |", rank + 1);
        for file in 0..8 {
            let square = Square::new_square_from_index(file, rank).unwrap();
            let piece = board.get_piece_square(&square);
            print!("{} ", ChessPiece::letter_from_piece(piece));
        }
        println!();
    }
    println!("------------------");
    println!("  |a b c d e f g h");
}
