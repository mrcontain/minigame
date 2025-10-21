use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct SimplePlayer {
    pub player_id: i32,
    pub player_name: String,
}

impl SimplePlayer {
    pub fn new(player_id: i32, player_name: String) -> Self {
        Self {
            player_id,
            player_name,
        }
    }

    pub async fn add_player(
        pool: &Pool<Postgres>,
        player_id: i32,
        player_name: String,
    ) -> Result<()> {
        let result =
            sqlx::query("INSERT INTO player_info (player_id, player_name) VALUES ($1, $2)")
                .bind(player_id)
                .bind(player_name)
                .execute(pool)
                .await?;
        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("添加玩家失败"));
        }
        Ok(())
    }
}
