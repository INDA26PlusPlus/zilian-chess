# zilian-chess
This is a backend chess library written in Rust!
---
## Construction:
This chess library is made using the mailbox design. It stores all 64 squares in a 8x8 nested array, each nested array has a ChessPiece structure. The good thing about each piece being unique and their own structure is that we can store additional information in the piece such as:

- Has the piece moved?
- What color is the piece?
- What type of piece is it?

... to be continued
Insert rambling notes:
separates the board from the API methods, that way we can completely remake the backend but keep the same Square structure for referring to a specific square on the board and the program will run just the same.

## Usage:

The API works through the ChessGame package, this enables a user to make a new game, with a new board and make moves on that board.
Every time a move is made through the game package, it checks with a movement file to ensure that the move is legal and then allows the user to execute said move.