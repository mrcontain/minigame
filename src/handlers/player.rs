use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{AppState, SimplePlayer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddPlayerRequest {
    pub player_id: i32,
    pub player_name: String,
}
pub async fn add_player(
    State(state): State<AppState>,
    Json(request): Json<AddPlayerRequest>,
) -> impl IntoResponse {
    match SimplePlayer::add_player(&state.pool, request.player_id, request.player_name).await {
        Ok(_) => {
            return (StatusCode::OK, "玩家添加成功").into_response();
        }
        Err(e) => {
            error!("❌ [add_player] 添加玩家失败 - 错误: {}", e);
            return (StatusCode::BAD_REQUEST, "添加玩家失败").into_response();
        }
    }
}