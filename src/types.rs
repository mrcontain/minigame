use crate::dto::MessageResponse;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Room {
    pub room_id: i32,
    pub players: Vec<Player>,
    pub car_id: i32,
    pub weather_id: i32,
    pub background_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub player_id: i32,
    pub player_name: String,
    pub car_id: i32,
    pub weather_id: i32,
    pub background_id: i32,
}

pub type room_broadcast_couple = (
    broadcast::Sender<MessageResponse>,
    broadcast::Receiver<MessageResponse>,
);
