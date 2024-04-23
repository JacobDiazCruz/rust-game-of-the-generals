use std::error::Error;

use crate::{ player::Player };

pub async fn create_player(player: &Player, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO player (player_name, player_color) VALUES ($1, $2)";

    sqlx::query(query).bind(&player.player_name).bind(&player.player_color).execute(pool).await?;

    Ok(())
}