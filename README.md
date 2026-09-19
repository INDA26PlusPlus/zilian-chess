# NOTE
Might add some extra methods. This wouldn't effect any existing methods so API is still fully functioning at its current state


# ChessBox
Chess library made in **RUST**!
The library uses a mailbox design (hence the name), specifically a 2d (8x8) nested array. Where each value is a ChessPiece structure.

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

## Chess board

The board is a structure containing a a nested array:
```rust
pub struct Board {
    squares: [[Option<ChessPiece>; 8]; 8]
}
```
Since we referring to a specific square using indices `squares[x][y]` can be annoying and doesn't fit all use cases, we make use of a separate structure to refer to each square:

```rust
pub struct Square {
    file: i8,
    rank: i8
}
```
So if we want to make use of notation to represent a given square on the board we can use one of the square constrictors
```rust
pub fn new_square_from_notation_str(notation_string: &str) -> Option<Self>

pub fn new_square_from_notation(file: char, rank: i8) -> Option<Self> 

pub fn new_square_from_index(file: i8, rank: i8) -> Option<Self> 
```
Depending of what is needed, any one of these methods all represent a `Square` with a file and rank. We use options so that if the square we're trying to make is outside the bounds of the board, we simply get a `Ǹone`.

### Pieces
The board is filled with `Option<ChessPiece>` elements in each square. The `ChessPiece` structure contains the following:
```rust
pub struct ChessPiece {
    pub piece_type: PieceType,
    is_white:   bool,
    has_moved:  bool,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
```
We use `Option<ChessPiece>` so that `None` can represent an unoccupied square. If we want to find out information about a piece we must first check that it's not empty, then we can check what `PieceType` it has and what its color is.

# Game
The previous structures don't keep track of movement logic, who's turn it is, ect. For that we need to tie a board to a `ChessGame` structure:
```rust
pub struct ChessGame {
    board_state: Board,
    is_white_turn: bool,
    movement_logic: Move,
}
```
All things are handled through the `ChessGame`, so to start a standard chess game use the following method:
```rust
pub fn new_standard_game() -> Self 
```
To make a move we use the following method:
```rust
make_move(&mut self, start: Option<Square>, stop: Option<Square>) -> Result<(), MoveError>
```
When making a move we declare the square we want to "start" the move from, and the square we want to "stop" on. The method then checks if the move is valid and will either make the move, or return a `MoveError`.

## Errors
The types of `MoveError`s are as follows:
| MoveError | Description |
|-----------|-------------|
| InvalidNotation   | `Option<Square>` is `None` |
| EmptySquare       | start square had `Option<ChessPiece>` be empty |
| NotYourPiece      | start square not same color as whose turn it is  |
| CantSelfHarm      | stops square is occupied by your own piece |
| IllegalMove       | Illegal for piece on start to move to stop |
| HangsKing         | Would leave your own king in check |

# Example:
Heres an example of how the API could be used:
```rust
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
            // Checking color of piece
            if piece.is_white() {
                println!("The piece on {}, {} is WHITE!", example_square.file(), example_square.rank())
            }
            // Checking type of piece
            if piece.piece_type() == PieceType::King {
                println!("The piece on {}, {} is a KING!", example_square.file(), example_square.rank())
            }
        }
        None => println!("The piece on {}, {} is empty!", example_square.file(), example_square.rank())
    }
```

# Additional information
There are several more methods at your disposal within the library. Some notable ones are:
```rust
pub fn make_move_notation(&mut self, start: &str, stop: &str) -> Result<(), MoveError>

pub fn is_white_turn(&self) -> bool

pub fn turn_text(&self) -> &str

pub fn in_check(&self) -> bool

pub fn in_checkmate(&self) -> bool

pub fn in_stalemate(&self) -> bool

pub fn get_piece_file_rank(&self, file: i8, rank: i8) -> Option<ChessPiece>

pub fn would_be_capture(&self, start: &Square, stop: &Square) -> bool

pub fn piece_from_letter(letter: char, has_moved: bool) -> Option<Self>

pub fn letter_from_piece(piece: Option<Self>) -> char
```

I plan on making some minor changes with the underlying movement logic, this shouldn't effect how one would use the API since everything handled is handled through the `ChessGame`.