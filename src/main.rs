pub mod player;

use crate::player::{ PlayerBuilder, PlayerColors };
// create a model for a player with their pieces
// instantiate the model by defining: let white_player = Player::new()
// create a model for the board
// create the board with its users
// Board::new(white_player, black_player)
// add the logic in the board model
// create an API handler where it accepts 2 params
// 1. ULDR
// 2. the player who moves
// 3. piece
// features:
// - move the piece to its desired direction
// - if the piece meets a piece, then if the piece is > than the piece that it tries to remove, remove the piece, else remove the param piece
fn main() {
    println!("Hello, world!");
    let white = PlayerBuilder::new("CJ".to_string(), PlayerColors::White).build();
    println!("Player: {:#?}", white)
}
