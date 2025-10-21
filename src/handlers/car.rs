use axum::{extract::State, response::IntoResponse};
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;

use crate::{AppState, ChangeCarRequest, MessageType};
use axum::Json;
use tracing::debug;
use tracing::error;

pub async fn change_car(
    State(state): State<AppState>,
    Json(request): Json<ChangeCarRequest>,
) -> impl IntoResponse {
    let room_info = (*state).room_info.get_mut(&request.room_id);
    let room_info_clone;
    match room_info {
        Some(mut room) => {
            (*room).cars.iter_mut().for_each(|car| {
                car.player_ids.retain(|id| *id != request.player_id);
                if car.car_id == request.car_id {
                    car.player_ids.push(request.player_id);
                }
            });
            room_info_clone = room.clone();
        }
        None => {
            return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
        }
    }
    match (*state).room_broadcast_couple.get(&request.room_id) {
        Some(couple) => {
            match couple.0.send(MessageType::Sync(room_info_clone)) {
                Ok(_) => {
                    debug!("✅ [broadcast_to_ws] 同步消息广播成功");
                }
                Err(e) => {
                    error!("❌ [broadcast_to_ws] 同步消息广播失败 - 错误: {}", e);
                    return (StatusCode::BAD_REQUEST, "房间退出失败").into_response();
                }
            };
        }
        None => {
            return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
        }
    }
    (StatusCode::OK, "车辆更换成功").into_response()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeCarSkinRequest {
    pub room_id: i32,
    pub car_id: i32,
    pub skin_id: i32,
}

pub async fn change_car_skin(
    State(state): State<AppState>,
    Json(request): Json<ChangeCarSkinRequest>,
) -> impl IntoResponse {
    let room_info = (*state).room_info.get_mut(&request.room_id);
    match room_info {
        Some(mut room) => {
            (*room).cars.iter_mut().for_each(|car| {
                if car.car_id == request.car_id {
                    car.skin_id = request.skin_id;
                }
            });
            match (*state).room_broadcast_couple.get(&request.room_id) {
                Some(couple) => match couple.0.send(MessageType::Sync(room.clone())) {
                    Ok(_) => {
                        debug!("✅ [broadcast_to_ws] 同步消息广播成功");
                        return (StatusCode::OK, "车辆皮肤更换成功").into_response();
                    }
                    Err(e) => {
                        error!("❌ [broadcast_to_ws] 同步消息广播失败 - 错误: {}", e);
                        return (StatusCode::BAD_REQUEST, "房间退出失败").into_response();
                    }
                },
                None => {
                    return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
                }
            }
        }
        None => {
            return (StatusCode::BAD_REQUEST, "房间不存在").into_response();
        }
    }
}
