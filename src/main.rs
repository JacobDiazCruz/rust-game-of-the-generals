pub mod player;
pub mod game;
pub mod queries;
pub mod piece;

use dotenv::dotenv;
use queries::queries::create_game;
use std::error::Error;
use crate::queries::queries::create_player;
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

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    row: u32,
    col: u32,
}

fn valid_cells_to_move(cell: Cell, cell_to_move: Cell) -> Vec<String> {
    let up = format!("{}{}", cell.row + 1, cell.col);
    let down = format!("{}{}", cell.row - 1, cell.col);
    let left = format!("{}{}", cell.row, cell.col - 1);
    let right = format!("{}{}", cell.row, cell.col + 1);

    let mut valid_cells_to_move = Vec::new();

    // first row conditions
    if cell.row == 1 && cell.col == 1 {
        valid_cells_to_move.extend(vec![down, right]);
        return valid_cells_to_move;
    } else if cell.row == 1 && cell.col == 9 {
        valid_cells_to_move.extend(vec![down, left]);
        return valid_cells_to_move;
    } else if cell.row == 1 {
        valid_cells_to_move.extend(vec![down, left, right]);
        return valid_cells_to_move;
    }

    // last row conditions
    if cell.row == 8 && cell.col == 1 {
        valid_cells_to_move.extend(vec![up.clone(), right.clone()]);
        return valid_cells_to_move;
    } else if cell.row == 8 && cell.col == 9 {
        valid_cells_to_move.extend(vec![up.clone(), left.clone()]);
        return valid_cells_to_move;
    } else if cell.row == 8 {
        valid_cells_to_move.extend(vec![up.clone(), left.clone(), right.clone()]);
        return valid_cells_to_move;
    }

    // middle rows but last cols
    if cell.row != 1 && cell.row != 8 {
        if cell.col == 1 {
            valid_cells_to_move.extend(vec![up.clone(), down.clone(), right.clone()]);
            return valid_cells_to_move;
        } else if cell.col == 9 {
            valid_cells_to_move.extend(vec![up.clone(), down.clone(), left.clone()]);
            return valid_cells_to_move;
        }
    }

    // if middle rows and middle cols
    valid_cells_to_move.extend(vec![up.clone(), down.clone(), left.clone(), right.clone()]);
    valid_cells_to_move
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let sql_uri = std::env
        ::var("SQL_URI")
        .expect("SQL_URI must be set");

    let pool = sqlx::postgres::PgPool::connect(&sql_uri).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("wait");
    let player_one = PlayerBuilder::new("PLAYER_ONE".to_string(), "White".to_string()).build();
    let player_two = PlayerBuilder::new("PLAYER_TWO".to_string(), "Black".to_string()).build();
    create_player(&player_one, &pool).await?;
    create_player(&player_two, &pool).await?;
    let game = GameBuilder::new(player_one.clone(), player_two).build();
    let created_game = create_game(&game, &pool).await?;
    println!("created_game {:#?}", created_game);

    let cell = Cell {
        row: 3,
        col: 5,
    };
    let cell_to_move = String::from("36");
    let valid_cells_to_move = valid_cells_to_move(cell, Cell { row: 3, col: 6 });
    if !valid_cells_to_move.contains(&cell_to_move) {
        println!("invalid cell to move, must be up down left or right only!");
    }

    Ok(())
}
