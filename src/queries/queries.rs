use std::error::Error;
use uuid::Uuid;
use sqlx::Row;

use crate::player::Player;
use crate::piece::Piece;
use crate::game::Game;

pub async fn create_player(player: &Player, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO player (id, player_name, player_color) VALUES ($1, $2, $3)";

    sqlx::query(query).bind(&player.id).bind(&player.player_name).bind(&player.player_color).execute(pool).await?;

    Ok(())
}

pub async fn create_game(game: &Game, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4();
    let query = "INSERT INTO game (id, white_player_id, black_player_id, turn_player_id, winner_player_id) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(&id)
        .bind(&game.white_player_id)
        .bind(&game.black_player_id)
        .bind(&game.turn_player_id)
        .bind(&game.winner_player_id)
        .execute(pool)
        .await?;

    Ok(())
}


pub async fn create_piece(piece: &Piece, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO piece (name, player_id, game_id, square) VALUES ($1, $2, $3, $4)";

    sqlx::query(query)
        .bind(&piece.name)
        .bind(&piece.player_id)
        .bind(&piece.game_id)
        .bind(&piece.square)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_square_piece(selected_square: String, conn: &sqlx::PgPool) -> Result<Option<Piece>, Box<dyn Error>> {
    let q = "SELECT name, square FROM games WHERE square = $1";
    let query = sqlx::query(q).bind(selected_square);

    let maybe_row = query.fetch_optional(conn).await?;

    let piece = maybe_row.map(|row| {
        Piece{
            name: row.get("name"),
            player_id: row.get("player_id"),
            game_id: row.get("game_id"),
            square: row.get("square"),
        }
    });

    Ok(piece)
}