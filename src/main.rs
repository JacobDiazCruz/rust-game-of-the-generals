pub mod player;
pub mod board;

use crate::player::{ PlayerBuilder, PlayerColors };
use crate::board::BoardBuilder;

// Model
// Player:
//     user_id,
//     name,
//     email

// Game:
//     white: {
//         user_id,
//         pieces: Piece[]
//     },
//     black: {
//         user_id,
//         pieces: Piece[]
//     },
//     turn: white,
//     winner: white,
//     status: "ONGOING" | "DONE"

// Piece:
//     name,
//     size,
//     code,
//     cell

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
//         - else if your piece is stronger than the opposing piece, delete the opposing piece, and add your piece to the cell
//         - else delete your piece
//     4. if the piece is a "flag", and the desired cell is in the last row of the opposing team, update game "winner"
fn main() {
    println!("Hello, world!");
    let white_player = PlayerBuilder::new("PLAYER_1".to_string(), PlayerColors::White).build();
    let black_player = PlayerBuilder::new("PLAYER_2".to_string(), PlayerColors::Black).build();
    let board = BoardBuilder::new(white_player, black_player);
}
