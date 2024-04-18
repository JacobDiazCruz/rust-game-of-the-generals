pub mod player;
pub mod board;
pub mod game;

use crate::player::{ PlayerBuilder, PlayerColors };
use crate::board::BoardBuilder;

// Board
// [A1][A2][A3][A4][A5][A6][A7][A8][A9]
// [B1][B2][B3][B4][B5][B6][B7][B8][B9]
// [C1][C2][C3][C4][C5][C6][C7][C8][C9]
// [D1][D2][D3][D4][D5][D6][D7][D8][D9]
// [E1][E2][E3][E4][E5][E6][E7][E8][E9]
// [F1][F2][F3][F4][F5][F6][F7][F8][F9]
// [G1][G2][G3][G4][G5][G6][G7][G8][G9]
// [H1][H2][H3][H4][H5][H6][H7][H8][H9]

// Model
// Player:
//     user_id,
//     name,
//     email

// Game:
//     white: {
//         user_id,
//         pieces: Piece[],
//         removed_pieces: Piece[]
//     },
//     black: {
//         user_id,
//         pieces: Piece[],
//         removed_pieces: Piece[]
//     },
//     turn: white,
//     winner: white,
//     status: "ONGOING" | "DONE"

// Piece:
//     name,
//     size,
//     code,
//     cell

// // Socket endpoints
// // /game : creates a game with 2 players
// // /game/:id : update game's turn, winner, status, and pieces moved.

// // Game flow
// The game starts and assign 2 players to the Game model
// The players will be given 20 seconds to reposition their pieces
//     - The system will update each piece cell (move_piece())
// After 20 seconds, the game will choose the player turn randomly
// The player who has the turn will now move a piece.
//     - The system will update each piece cell (move_piece())

// // Functions
// move_piece() will have the following steps:
//     1. if the desired cell does not follow ULDR directions from your current cell; throw Err;
//     2. if the desired cell has an ally piece sitting on it, throw Err;
//     3. if the desired cell has an opposing piece:
//         - if your piece is same as the opposing piece, delete both piece
//         - else if your piece is stronger than the opposing piece, move the opposing piece to "removed_pieces", and add your piece to the cell
//         - else delete your piece
//     4. if the piece is a "flag", and the desired cell is in the last row of the opposing team, update game "winner"

fn main() {
    println!("Hello, world!");
    let white_player = PlayerBuilder::new("PLAYER_1".to_string(), PlayerColors::White).build();
    let black_player = PlayerBuilder::new("PLAYER_2".to_string(), PlayerColors::Black).build();
    let board = BoardBuilder::new(white_player, black_player);
}
