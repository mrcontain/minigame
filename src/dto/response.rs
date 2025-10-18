use serde::{Deserialize, Serialize};

use crate::Room;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageResponse {
    pub player_id : i32,
    pub content : String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageType {
    Text(MessageResponse),
    Emoji(MessageResponse),
    Sync(Room),
    Quit(i32,i32)
}