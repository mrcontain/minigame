use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageResponse {
    pub player_id : i32,
    pub content : String,
}