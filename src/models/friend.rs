use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::SimplePlayer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Friend {
    pub master_id: i32,
    pub friend_ids: Vec<SimplePlayer>,
}

impl Friend {
    pub fn new(master_id: i32) -> Self {
        Self {
            master_id,
            friend_ids: vec![],
        }
    }

    pub async fn add_friend(pool: &Pool<Postgres>, master_id: i32, friend_id: i32) -> Result<()> {
        // 双向添加好友
        let result1 =
            sqlx::query("INSERT INTO friend_mapping (master_id, friend_id) VALUES ($1, $2)")
                .bind(master_id)
                .bind(friend_id)
                .execute(pool)
                .await?;
        
        let result2 =
            sqlx::query("INSERT INTO friend_mapping (master_id,friend_id) VALUES ($1, $2)")
                .bind(friend_id)
                .bind(master_id)
                .execute(pool)
                .await?;
        if result1.rows_affected() == 0 && result2.rows_affected() == 0 {
            return Err(anyhow::anyhow!("添加好友失败"));
        }
        Ok(())
    }

    pub async fn remove_friend(
        pool: &Pool<Postgres>,
        master_id: i32,
        friend_id: i32,
    ) -> Result<()> {
        let result =
            sqlx::query(r#"DELETE FROM friend_mapping WHERE master_id = $1 AND friend_id = $2"#)
                .bind(master_id)
                .bind(friend_id)
                .execute(pool)
                .await?;
        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("删除好友失败"));
        }
        Ok(())
    }

    pub async fn get_friends(pool: &Pool<Postgres>, master_id: i32) -> Result<Self> {
        let friend_ids: Vec<SimplePlayer> = sqlx::query_as("SELECT player_id, player_name FROM player_info WHERE player_id IN (SELECT friend_id FROM friend_mapping WHERE master_id = $1)")
            .bind(master_id)
            .fetch_all(pool)
            .await?;

        Ok(Self {
            master_id,
            friend_ids,
        })
    }

}
