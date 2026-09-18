# ChessBox
Chess library made in **RUST**!
The library uses a mailbox design (hence the name), specifically a 2d (8x8) nested array. Where each value is a ChessPiece structure.

## Board visualization

The array is indexed `squares[file][rank]`, both 0-7. File `a` is 0 through file `h` at 7, rank "1" is 0 through rank "8" at 7:

```
8 | . . . . . . . .
7 | . . . . . . . .
6 | . . . . . . . .
5 | . . . . . . . .
4 | . . . . . . . .
3 | . . . . . . . .
2 | . . . . . . . .
1 | . . . . . . . .
  +----------------
    a b c d e f g h
```

So e4 (file `e`, rank `4`) lives at `squares[4][3]`. Though you won't need to index the array yourself as well see in `Square`.

# Dependencies
To use this library, make sure you mark it as a dependency in Cargo.toml. The package name is `chess-box`, so the key on the left has to match that.

Either through file path:
```toml
[dependencies]
chess-box = { path = "../chess-box" }
```
Or directly through git:
```toml
[dependencies]
chess-box = { git = "https://github.com/INDA26PlusPlus/zilian-chess.git" }
```

# Usage
To get started with the library simply make a new game with a new, standard starting board:
```rust
use chess_box::game::ChessGame;

let mut game = ChessGame::new_standard_game();
```

If you want a custom starting position instead, build your own `Board` and pass it (plus who moves first) to `ChessGame::new_game`:
```rust
use chess_box::board::Board;
use chess_box::game::ChessGame;

let board = Board::new_starting_board(); // or Board::board_from_strings([...])
let mut game = ChessGame::new_game(board, true); // true = white moves first
```

Since the way we want to refer to a given square on the board may differ depending on use case, the ChessBox library uses a `Square` struct to represent a position on the board without exposing the raw array indices. You build one from chess notation:

```rust
use chess_box::board::Square;

let square = Square::square_from_notation_str("e4"); // -> Option<Square>
```

Construction returns `None` for anything out of range of the board instead of panicking, so it's safe to build directly from raw user input. `Square` also implements `PartialEq`, so you can compare two squares directly with `==`.

To move pieces simply use the built in game.make_move method, 
```rust
match game.make_move("e2", "e4") {
    Ok(()) => println!("Moved!"),
    Err(reason) => println!("Illegal move: {:?}", reason),
}
```
`make_move` takes chess notation for the start and stop squares, applies the move if it's legal, and switches the turn. On failure you get back a `MoveError` explaining why — see the Errors section below for details.

# Errors
`make_move` returns `Result<(), MoveError>`. `MoveError` is a proper enum rather than a loose string, so callers can match on exactly why a move was rejected instead of parsing text:

```rust
#[derive(Debug)]
pub enum MoveError {
    InvalidNotation(String), // Not a real square, e.g. "z9"
    EmptySquare(String),     // Nothing to move on the starting square
    NotYourPiece(String),    // Piece belongs to the other player
    CantSelfHarm,            // Destination is occupied by your own piece
    IllegalMove(String),     // Wrong shape for the piece, or blocked
    HangsKing,               // Would leave your own king in check
}
```

# Key methods
Methods you can use during the game:

- `game.board() -> &Board` — the current board
- `game.is_white_turn() -> bool` / `game.turn_text() -> &str` — whose turn it is
- `game.in_check() -> bool` — is the player to move currently in check?
- `game.in_checkmate() -> bool` — is the game over, with the player to move losing?
- `game.in_stalemate() -> bool` — is the game a draw with no legal moves left?

# Example
A very simple example for how one could setup a chess game using the library would be:
```rust
use std::io;
use chess_box::game::ChessGame;

fn main() {
    let mut game = ChessGame::new_standard_game();

    loop {
        println!("{}", game.turn_text());

        println!("Enter starting square:");
        let mut start = String::new();
        io::stdin().read_line(&mut start).expect("Failed to read line");

        println!("Enter destination square:");
        let mut stop = String::new();
        io::stdin().read_line(&mut stop).expect("Failed to read line");

        match game.make_move(start.trim(), stop.trim()) {
            Ok(()) => {
                if game.in_checkmate() {
                    println!("Checkmate!");
                    break;
                } else if game.in_stalemate() {
                    println!("Stalemate - draw.");
                    break;
                } else if game.in_check() {
                    println!("{} is in check!", game.turn_text());
                }
            }
            Err(reason) => println!("Illegal move: {:?}", reason),
        }
    }
}
```
