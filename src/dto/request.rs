use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct  JoinRoomRequest {
    pub room_id: i32,
    pub player_id: i32,
}