use std::error::Error;
use sqlx::Row;
use uuid::Uuid;

use crate::player::Player;
use crate::piece::Piece;
use crate::game::Game;

pub async fn create_player(player: &Player, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO player (id, player_name, player_color) VALUES ($1, $2, $3)";

    sqlx
        ::query(query)
        .bind(&player.id)
        .bind(&player.player_name)
        .bind(&player.player_color)
        .execute(pool).await?;

    Ok(())
}

pub async fn create_game(game: &Game, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "INSERT INTO game (id, white_player_id, black_player_id, turn_player_id, winner_player_id) VALUES ($1, $2, $3, $4, $5)";

    sqlx
        ::query(query)
        .bind(&game.id)
        .bind(&game.white_player_id)
        .bind(&game.black_player_id)
        .bind(&game.turn_player_id)
        .bind(&game.winner_player_id)
        .execute(pool).await?;

    Ok(())
}

pub async fn create_piece(piece: &Piece, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "INSERT INTO piece (id, rank, name, player_id, game_id, square) VALUES ($1, $2, $3, $4, $5, $6)";

    sqlx
        ::query(query)
        .bind(&piece.id)
        .bind(&piece.rank)
        .bind(&piece.name)
        .bind(&piece.player_id)
        .bind(&piece.game_id)
        .bind(&piece.square)
        .execute(pool).await?;

    Ok(())
}

pub async fn update_game_winner(player_id: &Uuid, game_id: &Uuid, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "UPDATE game SET winning_player_id = $1 WHERE id = $2";

    sqlx
        ::query(query)
        .bind(&player_id)
        .bind(&game_id)
        .execute(pool).await?;

    Ok(())
}

pub async fn update_piece_square(square: String, piece_id: Uuid, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "UPDATE piece SET square = $1 WHERE id = $2";

    sqlx
        ::query(query)
        .bind(&square)
        .bind(&piece_id)
        .execute(pool).await?;

    Ok(())
}

pub async fn get_piece(
    game_id: &Uuid,
    square: String,
    conn: &sqlx::PgPool
) -> Result<Option<Piece>, Box<dyn Error>> {
    let q = "SELECT * FROM piece WHERE square = $1 AND game_id = $2";
    let query = sqlx::query(q).bind(square).bind(game_id);

    let maybe_row = query.fetch_optional(conn).await?;

    let piece = maybe_row.map(|row| {
        Piece {
            id: row.get("id"),
            rank: row.get("rank"),
            name: row.get("name"),
            player_id: row.get("player_id"),
            game_id: row.get("game_id"),
            square: row.get("square")
        }
    });

    Ok(piece)
}

