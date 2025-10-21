use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use crate::{AppState, Friend};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddFriendRequest {
    pub master_id: i32,
    pub friend_id: i32,
}

pub async fn add_friend(
    State(state): State<AppState>,
    Json(request): Json<AddFriendRequest>,
) -> impl IntoResponse {
    match Friend::add_friend(&state.pool, request.master_id, request.friend_id).await {
        Ok(_) => {
            (StatusCode::OK, "好友添加成功").into_response()
        }
        Err(e) => {
            error!("❌ [add_friend] 添加好友失败 - 错误: {}", e);
            (StatusCode::BAD_REQUEST, "添加好友失败").into_response()
        }
    };
    (StatusCode::OK, "好友添加成功").into_response()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoveFriendRequest {
    pub master_id: i32,
    pub friend_id: i32,
}
pub async fn remove_friend(
    State(state): State<AppState>,
    Json(request): Json<RemoveFriendRequest>,
) -> impl IntoResponse {
    match Friend::remove_friend(&state.pool, request.master_id, request.friend_id).await {
        Ok(_) => {
            (StatusCode::OK, "好友删除成功").into_response()
        }
        Err(e) => {
            error!("❌ [remove_friend] 删除好友失败 - 错误: {}", e);
            (StatusCode::BAD_REQUEST, "删除好友失败").into_response()
        }
    };
    (StatusCode::OK, "好友删除成功").into_response()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetFriendsRequest {
    pub master_id: i32,
}
pub async fn get_friends(
    State(state): State<AppState>,
    Json(request): Json<GetFriendsRequest>,
) -> impl IntoResponse {
    match Friend::get_friends(&state.pool, request.master_id).await {
        Ok(friends) => {
            let json_response = json!({
                "master_id": friends.master_id,
                "friend_ids": friends.friend_ids,
            });
            (StatusCode::OK, Json(json_response)).into_response()
        }
        Err(e) => {
            error!("❌ [get_friends] 获取好友失败 - 错误: {}", e);
            (StatusCode::BAD_REQUEST, "获取好友失败").into_response()
        }
    };
}