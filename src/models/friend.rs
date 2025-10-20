#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Friend {
    pub master_id: i32,
    pub friend_ids: Vec<i32>,
}


impl Friend {
    pub fn new(master_id: i32) -> Self {
        Self {
            master_id,
            friend_ids: vec![],
        }
    }

    pub fn add_friend(pool: &Pool<Postgres>, master_id: i32, friend_id: i32) -> Result<()> {
        let conn = pool.get().await?;
        let query = "INSERT INTO friend_mapping (master_id, friend_id) VALUES ($1, $2)";
        let params = (master_id, friend_id);
        let _ = conn.execute(query, params).await?;
        Ok(())
    }

    pub fn remove_friend(pool: &Pool<Postgres>, master_id: i32, friend_id: i32) -> Result<()> {
        let conn = pool.get().await?;
        let query = "DELETE FROM friend_mapping WHERE master_id = $1 AND friend_id = $2";
        let params = (master_id, friend_id);
        let _ = conn.execute(query, params).await?;
        Ok(())
    }

    pub fn get_friends(pool: &Pool<Postgres>, master_id: i32) -> Result<Vec<i32>> {
        let conn = pool.get().await?;
        let query = "SELECT friend_id FROM friend_mapping WHERE master_id = $1";
        let params = (master_id,);
        let rows = conn.query(query, params).await?;
        let friends = rows.iter().map(|row| row.get("friend_id")).collect();
        Ok(friends)
    }
}