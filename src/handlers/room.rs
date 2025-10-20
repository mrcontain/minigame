use axum::{extract::State, response::IntoResponse};
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast;

use super::handle_broadcast_to_ws;
use super::handle_ws_to_broadcast;
use crate::MessageType;
use crate::QuitRoomRequest;
use crate::{AppState, Car, Player, Room};
use axum::Json;
use tracing::debug;
use tracing::error;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub player_id: i32,
    pub player_name: String,
    pub car_id: i32,
    pub weather_id: i32,
    pub background_id: i32,
}
pub async fn create_room(
    State(state): State<AppState>,
    Json(request): Json<CreateRoomRequest>,
) -> impl IntoResponse {
    let room_id = request.player_id;
    if state.inner.room_info.get(&room_id).is_none() {
        let cars = vec![];
        state.inner.room_info.insert(
            room_id,
            Room {
                room_id,
                players: vec![],
                cars: cars,
                weather_id: request.weather_id,
                background_id: request.background_id,
            },
        );
    } else {
        return (StatusCode::BAD_REQUEST, "房间已存在").into_response();
    }
    if state.inner.room_broadcast_couple.get(&room_id).is_none() {
        let (tx, rx) = broadcast::channel(100);
        state.inner.room_broadcast_couple.insert(room_id, (tx, rx));
    }

    // 返回json
    let json = json!({
        "room_id": room_id,
        "content": "房间创建成功",
    });
    (StatusCode::OK, Json(json)).into_response()
}

pub async fn quit_room(
    State(state): State<AppState>,
    Json(request): Json<QuitRoomRequest>,
) -> impl IntoResponse {
    let room_id = request.room_id;
    let quit_player_id = request.player_id;
    if (*state).room_info.get(&room_id).is_none() {
        return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
    }
    if state.inner.room_broadcast_couple.get(&room_id).is_none() {
        return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
    }
    let couple = match state.inner.room_broadcast_couple.get(&room_id) {
        Some(couple) => couple,
        None => {
            return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
        }
    };
    let mut room_info = match state.inner.room_info.get_mut(&room_id) {
        Some(room) => room,
        None => {
            return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
        }
    };
    if let Some(pos) = room_info
        .players
        .iter()
        .position(|p| p.player_id == quit_player_id)
    {
        // 直接取出被删除的玩家（拿到所有权，不用 clone）
        let player = room_info.players.remove(pos);

        // 删除该玩家对应的 car
        room_info.cars.retain(|c| c.car_id != player.car_id);

        // 从所有 car 的 player_ids 里移除这个玩家 id
        for car in &mut room_info.cars {
            car.player_ids.retain(|id| *id != quit_player_id);
        }
    } else {
        return (StatusCode::BAD_REQUEST, "玩家不存在").into_response();
    }
    (*state).normal_quit_room.insert(quit_player_id, ());

    let tx = couple.0.clone();
    match tx.send(MessageType::Quit(request.player_id, room_id)) {
        Ok(_) => {
            debug!("✅ [broadcast_to_ws] 退出消息广播成功");
        }
        Err(e) => {
            error!("❌ [broadcast_to_ws] 退出消息广播失败 - 错误: {}", e);
            return (StatusCode::BAD_REQUEST, "房间退出失败").into_response();
        }
    }
    (StatusCode::OK, "房间退出成功").into_response()
}
