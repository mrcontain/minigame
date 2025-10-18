use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageResponse {
    pub player_id : i32,
    pub content : String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageType {
    Text(MessageResponse),
    Emoji(MessageResponse),
}