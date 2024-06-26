pub mod player;
pub mod game;
pub mod queries;
pub mod piece;

use piece::{PieceBuilder, Directions};
use uuid::Uuid;
use dotenv::dotenv;
use queries::queries::{ create_game, get_piece, update_piece_square };
use std::error::Error;
use crate::piece::Piece;
use crate::queries::queries::{ create_player, create_piece, update_game_winner };
use crate::{ player::PlayerBuilder, game::GameBuilder };

// Board
// [11][12][13][14][15][16][17][18][19]
// [21][22][23][24][25][26][27][28][29]
// [31][32][33][34][35][36][37][38][39]
// [41][41][42][43][45][45][46][47][48]
// [51][52][53][54][55][56][57][58][59]
// [61][62][63][64][65][66][67][68][69]
// [71][72][73][74][75][76][77][78][79]
// [81][82][83][84][85][86][87][88][89]

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
//     square

// // Socket endpoints
// // /game : creates a game with 2 players
// // /game/:id : update game's turn, winner, status, and pieces moved.

// // Game flow
// The game starts and assign 2 players to the Game model
// The players will be given 20 seconds to reposition their pieces
//     - The system will update each piece square (move_piece())
// After 20 seconds, the game will choose the player turn randomly
// The player who has the turn will now move a piece.
//     - The system will update each piece square (move_piece())

// // Functions
// move_piece() will have the following steps:
//     1. if the desired square does not follow ULDR directions from your current square; throw Err;
//     2. if the desired square has an ally piece sitting on it, throw Err;
//     3. if the desired square has an opposing piece:
//         - if your piece is same as the opposing piece, delete both piece
//         - else if your piece is stronger than the opposing piece, move the opposing piece to "removed_pieces", and add your piece to the square
//         - else delete your piece
//     4. if the piece is a "flag", and the desired square is in the last row of the opposing team, update game "winner"
#[derive(Debug, Clone, Copy)]
pub struct Square {
    row: u32,
    col: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let sql_uri = std::env::var("SQL_URI").expect("SQL_URI must be set");

    let pool = sqlx::postgres::PgPool::connect(&sql_uri).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // create player
    let player_one = PlayerBuilder::new(
        Uuid::new_v4(),
        "PLAYER_ONE".to_string(),
        "White".to_string()
    ).build();
    let player_two = PlayerBuilder::new(
        Uuid::new_v4(),
        "PLAYER_TWO".to_string(),
        "Black".to_string()
    ).build();

    create_player(&player_one, &pool).await?;
    create_player(&player_two, &pool).await?;

    // create game
    let game = GameBuilder::new(Uuid::new_v4(), player_one.clone(), player_two).build();
    create_game(&game, &pool).await?;

    // create game pieces
    let board_pieces = PieceBuilder::create_board_pieces(player_one.id.unwrap(), game.id.unwrap());

    for piece in board_pieces.iter() {
        create_piece(piece, &pool).await?;
    }

    println!("hey");

    // Example of moving a piece
    // @Note: Move this to a socket function
    let dummy_two_star_general = PieceBuilder::new(
        4,
        "Two Star General".to_string(),
        player_one.id.unwrap().clone(),
        game.id.unwrap().clone(),
        "13".to_string()
    ).build();
    move_piece(
        &game.id.unwrap(),
        &player_one.id.unwrap(),
        dummy_two_star_general,
        Square { row: 1, col: 3 },
        "11".to_string(),
        &pool
    ).await?;

    Ok(())
}

async fn move_piece(
    game_id: &Uuid,
    player_id: &Uuid,
    my_piece: Piece,
    current_square: Square,
    destination_square: String,
    conn: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> {
    // get all valid squares to move "ULDR" direction
    let square_to_move = String::from("36");
    let directions = Directions {
        up: format!("{}{}", current_square.row + 1, current_square.col),
        down: format!("{}{}", current_square.row - 1, current_square.col),
        left: format!("{}{}", current_square.row, current_square.col - 1),
        right: format!("{}{}", current_square.row, current_square.col + 1)
    };
    let valid_squares_to_move = Piece::valid_squares_to_move(&current_square, &directions);
    if !valid_squares_to_move.contains(&&square_to_move) {
        println!("invalid square to move, must be up down left or right only!");
    }

    // check if there's a current piece in the desired square
    let current_piece = get_piece(game_id, destination_square.clone(), conn).await;
    match current_piece {
        Ok(Some(current_piece)) => {
            if current_piece.player_id == player_id.clone() {
                eprintln!("invalid square to move, there's an ally piece sitting in it.");
            }
            let enemy_piece = current_piece;
            compare_and_remove_piece(my_piece, enemy_piece, player_id, game_id, conn).await?;
            Ok(())
        }
        Ok(None) => {
            if 
                my_piece.name == "Flag".to_string() &&
                destination_square.clone().chars().next() == Some('1')
            {
                // if your piece is a flag and in the last row, you won.
                update_game_winner(player_id, game_id, conn).await?
            } else {
                // move your piece to the desired square
                // by updating the piece's square
                update_piece_square(destination_square, my_piece.id, &conn).await?
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

async fn compare_and_remove_piece(my_piece: Piece, enemy_piece: Piece, my_player_id: &Uuid, game_id: &Uuid, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if enemy_piece.name == my_piece.name {
        println!("Remove both pieces!");
    } else if enemy_piece.rank > 0 {
        // enemy piece is an officer
        if enemy_piece.rank > my_piece.rank {
            update_piece_square("00".to_string(), my_piece.id, pool).await?;
        } else if my_piece.name == "Private".to_string() {
            update_piece_square("00".to_string(), my_piece.id, pool).await?;
        } else if my_piece.name == "Spy".to_string() {
            update_piece_square("00".to_string(), enemy_piece.id, pool).await?;
        }
    } else if enemy_piece.name == "Spy".to_string() {
        // enemy is a spy
        if my_piece.rank > 0 {
            update_piece_square("00".to_string(), my_piece.id, pool).await?;
        } else if my_piece.name == "Private".to_string() {
            update_piece_square("00".to_string(), enemy_piece.id, pool).await?;
        }
    } else if enemy_piece.name == "Private".to_string() {
        // enemy is a private
        if my_piece.rank > 0 {
            update_piece_square("00".to_string(), enemy_piece.id, pool).await?;
        } else if my_piece.name == "Spy".to_string() {
            update_piece_square("00".to_string(), my_piece.id, pool).await?;
        }
    } else if enemy_piece.name == "Flag".to_string() {
        // enemy is flag
        if my_piece.name == "Flag".to_string() {
            // you attacked the enemy flag with your flag, you won.
            update_game_winner(my_player_id, game_id, pool).await?
        } else {
            // if your piece is a flag and in the last row, you won.
            update_game_winner(my_player_id, game_id, pool).await?
        }
    }
    Ok(())
}
