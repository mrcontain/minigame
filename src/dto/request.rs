use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct  JoinRoomRequest {
    pub room_id: i32,
    pub player_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct  QuitRoomRequest {
    pub room_id: i32,
    pub player_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeCarRequest {
    pub room_id: i32,
    pub player_id: i32,
    pub car_id: i32,
}