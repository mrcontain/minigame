use std::{collections::HashMap, hash::Hash};

use axum::{
    Json,
    body::Bytes,
    extract::{
        Path, Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use serde_json::json;
use tracing::{debug, error, info};

use crate::{AppState, Player};
use crate::{
    Car,
    dto::{JoinRoomRequest, MessageResponse},
};

// WebSocket处理函数
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(paramas): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let Some(player_id) = paramas.get("player_id") else {
        error!("缺少player_id参数");
        return (StatusCode::BAD_REQUEST, "缺少room_id参数").into_response();
    };
    let player_id = match player_id.parse::<i32>() {
        Ok(player_id) => player_id,
        Err(_) => {
            error!("player_id参数格式错误");
            return (StatusCode::BAD_REQUEST, "player_id参数格式错误").into_response();
        }
    };
    let Some(room_id) = paramas.get("room_id") else {
        error!("缺少room_id参数");
        return (StatusCode::BAD_REQUEST, "缺少room_id参数").into_response();
    };
    let room_id = match room_id.parse::<i32>() {
        Ok(room_id) => room_id,
        Err(_) => {
            error!("room_id参数格式错误");
            return (StatusCode::BAD_REQUEST, "room_id参数格式错误").into_response();
        }
    };
    let Some(player_name) = paramas.get("player_name") else {
        error!("缺少player_name参数");
        return (StatusCode::BAD_REQUEST, "缺少player_name参数").into_response();
    };
    let player_name = match player_name.parse::<String>() {
        Ok(player_name) => player_name,
        Err(_) => {
            error!("player_name参数格式错误");
            return (StatusCode::BAD_REQUEST, "player_name参数格式错误").into_response();
        }
    };
    let Some(car_id) = paramas.get("car_id") else {
        error!("缺少car_id参数");
        return (StatusCode::BAD_REQUEST, "缺少car_id参数").into_response();
    };
    let car_id = match car_id.parse::<i32>() {
        Ok(car_id) => car_id,
        Err(_) => {
            error!("car_id参数格式错误");
            return (StatusCode::BAD_REQUEST, "car_id参数格式错误").into_response();
        }
    };
    let Some(weather_id) = paramas.get("weather_id") else {
        error!("缺少weather_id参数");
        return (StatusCode::BAD_REQUEST, "缺少weather_id参数").into_response();
    };
    let weather_id = match weather_id.parse::<i32>() {
        Ok(weather_id) => weather_id,
        Err(_) => {
            error!("weather_id参数格式错误");
            return (StatusCode::BAD_REQUEST, "weather_id参数格式错误").into_response();
        }
    };
    let Some(background_id) = paramas.get("background_id") else {
        error!("缺少background_id参数");
        return (StatusCode::BAD_REQUEST, "缺少background_id参数").into_response();
    };
    let background_id = match background_id.parse::<i32>() {
        Ok(background_id) => background_id,
        Err(_) => {
            error!("background_id参数格式错误");
            return (StatusCode::BAD_REQUEST, "background_id参数格式错误").into_response();
        }
    };
    ws.on_upgrade(move |socket| async move {
        handle_websocket(
            socket,
            player_name,
            car_id,
            weather_id,
            background_id,
            player_id,
            room_id,
            state,
        )
        .await
    })
}

// 处理WebSocket连接
async fn handle_websocket(
    mut socket: WebSocket,
    player_name: String,
    car_id: i32,
    weather_id: i32,
    background_id: i32,
    player_id: i32,
    room_id: i32,
    state: AppState,
) {
    let mut room_info = match state.inner.room_info.get_mut(&room_id) {
        Some(room) => room,
        None => {
            error!("房间不存在");
            return;
        }
    };

    room_info.players.push(Player {
        player_id,
        player_name: player_name.clone(),
        car_id,
        weather_id,
        background_id,
    });
    room_info.cars.push(Car {
        car_id,
        player_ids: vec![player_id],
    });
    let first_json = json!({
        "room_info": room_info.clone(),
    });
    if socket
        .send(Message::Text(first_json.to_string().into()))
        .await
        .is_err()
    {
        error!("发送欢迎消息失败");
        return;
    }
    // 获取广播通道
    let room = match state.inner.room_broadcast_couple.get(&room_id) {
        Some(room) => room,
        None => {
            error!("房间广播pipeline不存在: {}", room_id);
            return;
        }
    };
    let tx = room.0.clone();
    let tx_clone = tx.clone();
    // 分离WebSocket发送和接收
    let (mut ws_sink, mut ws_stream) = socket.split();
    let content = format!("{}登录了房间", player_name);
    // 群发信息
    let ws_to_broadcast = tokio::spawn(async move {
        //文本帧使用json交互
        while let Some(Ok(msg)) = ws_stream.next().await {
            if let Message::Text(text) = msg {
                let json: serde_json::Value = match serde_json::from_str(&text) {
                    Ok(json) => json,
                    Err(_) => {
                        error!("解析JSON失败: {}", text);
                        continue;
                    }
                };
                let player_id = match json["player_id"].as_i64() {
                    Some(player_id) => player_id as i32,
                    None => {
                        error!("player_id字段不存在: {}", text);
                        continue;
                    }
                };
                let content = match json["content"].as_str() {
                    Some(content) => content.to_string(),
                    None => {
                        error!("content字段不存在: {}", text);
                        continue;
                    }
                };
                match tx.send(MessageResponse { player_id, content }) {
                    Ok(_) => {}
                    Err(_) => {
                        error!("发送消息失败: {}", text);
                        continue;
                    }
                }
            }
        }
    });

    // 监听broadcast pipeline如果收到消息则发送给客户端
    let broadcast_to_ws = tokio::spawn(async move {
        // 通知所有用户已登录
        match tx_clone.send(MessageResponse { player_id, content }) {
            Ok(_) => {}
            Err(_) => {
                error!("发送登录消息失败");
            }
        }
        let mut rx = tx_clone.subscribe();
        loop {
            match rx.recv().await {
                Ok(data) => match data {
                    MessageResponse { player_id, content } => {
                        if let Err(e) = ws_sink
                            .send(Message::Text(
                                json!({
                                    "player_id": player_id,
                                    "content": content,
                                })
                                .to_string()
                                .into(),
                            ))
                            .await
                        {
                            error!("发送消息失败: {}", e);
                        }
                    }
                },
                Err(e) => {
                    if let tokio::sync::broadcast::error::RecvError::Closed = e {
                        debug!("SSH通道已关闭");
                        let close_frame = Message::Close(Some(axum::extract::ws::CloseFrame {
                            code: 1008,
                            reason: "inactivetimeout".into(),
                        }));
                        if ws_sink.send(close_frame).await.is_err() {
                            error!("关闭帧发送失败");
                        }
                        break;
                    }
                }
            }
        }
    });

    // 等待任一任务结束
    tokio::select! {
        _ = ws_to_broadcast => {},
        _ = broadcast_to_ws => {},
    }
}
