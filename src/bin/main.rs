// Testing file to visually interpret the code
// Easier debugging and lets me use the API
// so i can see what I need to do to improve
// it from a user point-of-view.

use chess_box::board::{Board, Square};
use chess_box::pieces::{ChessPiece, PieceType};
use chess_box::game::ChessGame;
use std::io;


fn main () {

    // Makes a new game with standard layout
    let mut game: ChessGame = ChessGame::new_standard_game();

    // Make a new Square from a file and rank index
    // .unwrap() the Option<Square> since we know it's valid in this example
    let example_square: Square = Square::new_square_from_index(4, 0).unwrap();

    // Gets the value of the example_square on the board
    let example_piece: Option<ChessPiece> = game.board().get_piece_square(&example_square);

    // We can now check several things about the piece on the example square
    match example_piece {
        Some(piece) => {
            if piece.is_white() {
                println!("The piece on {}, {} is WHITE!", example_square.file(), example_square.rank())
            }
            if piece.piece_type() == PieceType::King {
                println!("The piece on {}, {} is a KING!", example_square.file(), example_square.rank())
            }
        }
        None => println!("The piece on {}, {} is empty!", example_square.file(), example_square.rank())
    }


    loop {
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
        match game.make_move_notation(
            &input_start.trim().to_lowercase(),
            &input_stop.trim().to_lowercase()
            ) {
                Ok(()) => {},
                Err(message) => println!("{:?}", message)
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
            if !piece.is_none() {
                if piece.unwrap().piece_type() == PieceType::Knight {
                    print!("ITS A KING")
                }
            }
        }
        println!();
    }
    println!("------------------");
    println!("  |a b c d e f g h");
}
