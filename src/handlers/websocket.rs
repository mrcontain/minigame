use std::{collections::HashMap, sync::Arc, time::Instant};

use axum::{
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use bytes::Bytes;
use futures::{SinkExt, StreamExt, future::join};
use http::StatusCode;
use log::info;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{debug, error};

use crate::{AppState, MessageType, Player, Room};
use crate::{Car, dto::MessageResponse};

// WebSocket处理函数
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(paramas): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    debug!(
        "🔌 [websocket_handler] 收到 WebSocket 连接请求，参数: {:?}",
        paramas
    );

    let Some(player_id) = paramas.get("player_id") else {
        error!("❌ [websocket_handler] 缺少player_id参数");
        return (StatusCode::BAD_REQUEST, "缺少room_id参数").into_response();
    };
    debug!(
        "✅ [websocket_handler] 获取到 player_id 参数: {}",
        player_id
    );

    let player_id = match player_id.parse::<i32>() {
        Ok(player_id) => {
            debug!("✅ [websocket_handler] player_id 解析成功: {}", player_id);
            player_id
        }
        Err(_) => {
            error!(
                "❌ [websocket_handler] player_id参数格式错误: {}",
                player_id
            );
            return (StatusCode::BAD_REQUEST, "player_id参数格式错误").into_response();
        }
    };
    let Some(room_id) = paramas.get("room_id") else {
        error!("❌ [websocket_handler] 缺少room_id参数");
        return (StatusCode::BAD_REQUEST, "缺少room_id参数").into_response();
    };
    debug!("✅ [websocket_handler] 获取到 room_id 参数: {}", room_id);

    let room_id = match room_id.parse::<i32>() {
        Ok(room_id) => {
            debug!("✅ [websocket_handler] room_id 解析成功: {}", room_id);
            room_id
        }
        Err(_) => {
            error!("❌ [websocket_handler] room_id参数格式错误: {}", room_id);
            return (StatusCode::BAD_REQUEST, "room_id参数格式错误").into_response();
        }
    };
    let Some(player_name) = paramas.get("player_name") else {
        error!("❌ [websocket_handler] 缺少player_name参数");
        return (StatusCode::BAD_REQUEST, "缺少player_name参数").into_response();
    };
    debug!(
        "✅ [websocket_handler] 获取到 player_name 参数: {}",
        player_name
    );

    let player_name = match player_name.parse::<String>() {
        Ok(player_name) => {
            debug!(
                "✅ [websocket_handler] player_name 解析成功: {}",
                player_name
            );
            player_name
        }
        Err(_) => {
            error!(
                "❌ [websocket_handler] player_name参数格式错误: {}",
                player_name
            );
            return (StatusCode::BAD_REQUEST, "player_name参数格式错误").into_response();
        }
    };
    let Some(car_id) = paramas.get("car_id") else {
        error!("❌ [websocket_handler] 缺少car_id参数");
        return (StatusCode::BAD_REQUEST, "缺少car_id参数").into_response();
    };
    debug!("✅ [websocket_handler] 获取到 car_id 参数: {}", car_id);

    let car_id = match car_id.parse::<i32>() {
        Ok(car_id) => {
            debug!("✅ [websocket_handler] car_id 解析成功: {}", car_id);
            car_id
        }
        Err(_) => {
            error!("❌ [websocket_handler] car_id参数格式错误: {}", car_id);
            return (StatusCode::BAD_REQUEST, "car_id参数格式错误").into_response();
        }
    };
    let Some(weather_id) = paramas.get("weather_id") else {
        error!("❌ [websocket_handler] 缺少weather_id参数");
        return (StatusCode::BAD_REQUEST, "缺少weather_id参数").into_response();
    };
    debug!(
        "✅ [websocket_handler] 获取到 weather_id 参数: {}",
        weather_id
    );

    let weather_id = match weather_id.parse::<i32>() {
        Ok(weather_id) => {
            debug!("✅ [websocket_handler] weather_id 解析成功: {}", weather_id);
            weather_id
        }
        Err(_) => {
            error!(
                "❌ [websocket_handler] weather_id参数格式错误: {}",
                weather_id
            );
            return (StatusCode::BAD_REQUEST, "weather_id参数格式错误").into_response();
        }
    };
    let Some(background_id) = paramas.get("background_id") else {
        error!("❌ [websocket_handler] 缺少background_id参数");
        return (StatusCode::BAD_REQUEST, "缺少background_id参数").into_response();
    };
    debug!(
        "✅ [websocket_handler] 获取到 background_id 参数: {}",
        background_id
    );

    let background_id = match background_id.parse::<i32>() {
        Ok(background_id) => {
            debug!(
                "✅ [websocket_handler] background_id 解析成功: {}",
                background_id
            );
            background_id
        }
        Err(_) => {
            error!(
                "❌ [websocket_handler] background_id参数格式错误: {}",
                background_id
            );
            return (StatusCode::BAD_REQUEST, "background_id参数格式错误").into_response();
        }
    };

    let Some(skin_id) = paramas.get("skin_id") else {
        error!("❌ [websocket_handler] 缺少skin_id参数");
        return (StatusCode::BAD_REQUEST, "缺少skin_id参数").into_response();
    };
    debug!(
        "✅ [websocket_handler] 获取到 skin_id 参数: {}",
        skin_id
    );
    let skin_id = match skin_id.parse::<i32>() {
        Ok(skin_id) => {
            debug!("✅ [websocket_handler] skin_id 解析成功: {}", skin_id);
            skin_id
        }
        Err(_) => {
            error!("❌ [websocket_handler] skin_id参数格式错误: {}", skin_id);
            return (StatusCode::BAD_REQUEST, "skin_id参数格式错误").into_response();
        }
    };

    debug!(
        "🚀 [websocket_handler] 所有参数验证成功，准备升级 WebSocket 连接 - player_id: {}, room_id: {}, player_name: {}",
        player_id, room_id, player_name
    );

    ws.on_upgrade(move |socket| async move {
        handle_websocket(
            socket,
            player_name,
            car_id,
            weather_id,
            background_id,
            player_id,
            room_id,
            skin_id,
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
    skin_id: i32,
    state: AppState,
) {
    debug!(
        "🎯 [handle_websocket] 进入 WebSocket 处理函数 - player_id: {}, room_id: {}, player_name: {}",
        player_id, room_id, player_name
    );
    let player = Player {
        player_id,
        player_name: player_name.clone(),
        car_id,
        weather_id,
        background_id,
    };
    let mut room_info = match state.inner.room_info.get_mut(&room_id) {
        Some(room) => {
            debug!(
                "✅ [handle_websocket] 房间信息获取成功 - room_id: {}",
                room_id
            );
            room
        }
        None => {
            error!("❌ [handle_websocket] 房间不存在 - room_id: {}", room_id);
            return;
        }
    };
    let first_json = {
        debug!(
            "🔍 [handle_websocket] 正在获取房间信息 - room_id: {}",
            room_id
        );
        debug!("📝 [handle_websocket] 添加玩家到房间");
        room_info.players.push(player.clone());
        debug!(
            "✅ [handle_websocket] 玩家添加成功，当前房间玩家数: {}",
            room_info.players.len()
        );

        debug!("🚗 [handle_websocket] 添加车辆到房间");
        room_info.cars.push(Car {
            car_id,
            skin_id,
            player_ids: vec![player_id],
        });
        debug!(
            "✅ [handle_websocket] 车辆添加成功，当前房间车辆数: {}",
            room_info.cars.len()
        );

        // 克隆数据用于返回，然后锁会在这个作用域结束时释放
        json!({
            "room_info": room_info.clone(),
        })
    };
    debug!(
        "📤 [handle_websocket] 准备发送欢迎消息，房间信息: {:?}",
        first_json
    );

    if socket
        .send(Message::Text(first_json.to_string().into()))
        .await
        .is_err()
    {
        error!("❌ [handle_websocket] 发送欢迎消息失败");
        return;
    }
    debug!("✅ [handle_websocket] 欢迎消息发送成功");
    // 获取广播通道
    debug!(
        "🔍 [handle_websocket] 正在获取广播通道 - room_id: {}",
        room_id
    );
    let room = match state.inner.room_broadcast_couple.get(&room_id) {
        Some(room) => {
            debug!(
                "✅ [handle_websocket] 广播通道获取成功 - room_id: {}",
                room_id
            );
            room
        }
        None => {
            error!(
                "❌ [handle_websocket] 房间广播pipeline不存在 - room_id: {}",
                room_id
            );
            return;
        }
    };
    let tx = room.0.clone();

    // 分离WebSocket发送和接收
    debug!("✂️ [handle_websocket] 分离 WebSocket 发送和接收通道");
    let (ws_sink, ws_stream) = socket.split();
    let arc_ws_sink = Arc::new(Mutex::new(ws_sink));

    let content = format!("{}登录了房间", player_name);
    debug!("📢 [handle_websocket] 准备广播登录消息: {}", content);

    // 群发信息 - 启动接收任务
    let ws_to_broadcast = tokio::spawn(handle_ws_to_broadcast(
        ws_stream,
        tx.clone(),
        room_id,
        player_id,
        state.clone(),
    ));

    // 监听broadcast pipeline如果收到消息则发送给客户端 - 启动发送任务
    let broadcast_to_ws = tokio::spawn(handle_broadcast_to_ws(
        arc_ws_sink.clone(),
        tx.clone(),
        player,
        content,
        state.clone(),
    ));
    let room_info_clone = room_info.clone();
    drop(room_info);

    let heartbeat_task = tokio::spawn(heartbeat_task(arc_ws_sink, player_id, state.clone()));
    match tx.send(MessageType::Sync(room_info_clone)) {
        Ok(_) => {
            debug!("✅ [broadcast_to_ws] 登录消息广播成功");
        }
        Err(e) => {
            error!("❌ [broadcast_to_ws] 发送登录消息失败 - 错误: {}", e);
        }
    };

    // 等待任一任务结束
    debug!("⏳ [handle_websocket] 等待任务结束...");
    match tokio::join!(ws_to_broadcast, broadcast_to_ws, heartbeat_task) {
        (Ok(_), Ok(_), Ok(_)) => {
            debug!("🛑 [handle_websocket] 所有任务已结束");
            debug!("room_id :{room_id} player_id :{player_id}");
        }
        (Err(e), _, _) => {
            error!(
                "❌ [handle_websocket] ws_to_broadcast 任务失败 - 错误: {}",
                e
            );
        }
        (_, _, Err(e)) => {
            error!(
                "❌ [handle_websocket] broadcast_to_ws 任务失败 - 错误: {}",
                e
            );
        }
        (_, Err(e), _) => {
            error!(
                "❌ [handle_websocket] heartbeat_task 任务失败 - 错误: {}",
                e
            );
        }
    }
    debug!("room_id :{room_id} player_id :{player_id}");
    drop(room);
    // 清理：从房间中移除玩家
    if room_id == player_id {
        match (*state).room_info.remove(&room_id) {
            Some(_) => {
                info!("room removed")
            }
            None => {
                error!("❌ [handle_websocket] 房间不存在");
                return;
            }
        };
        match (*state).room_broadcast_couple.remove(&room_id) {
            Some(couple) => {
                info!("couple removed");
                drop(couple);
            }
            None => {
                error!("❌ [handle_websocket] 房间广播管道不存在");
                return;
            }
        };
        debug!("🗑️ [handle_websocket] 房间 {} 已清空并删除", room_id);
    }
    (*state).normal_quit_room.remove(&player_id);
    debug!("👋 [handle_websocket] WebSocket 连接处理完成");
}

/// 处理从 WebSocket 接收的消息并广播到房间
pub async fn handle_ws_to_broadcast(
    mut ws_stream: futures::stream::SplitStream<WebSocket>,
    tx: tokio::sync::broadcast::Sender<MessageType>,
    room_id: i32,
    player_id: i32,
    state: AppState,
) {
    debug!("🚀 [ws_to_broadcast] 启动 WebSocket 接收任务");

    // 文本帧使用 json 交互
    while let Some(Ok(msg)) = ws_stream.next().await {
        // debug!("📨 [ws_to_broadcast] 收到 WebSocket 消息: {:?}", msg);

        match msg {
            Message::Text(text) => {
                debug!("📝 [ws_to_broadcast] 收到文本消息: {}", text);

                let json: serde_json::Value = match serde_json::from_str(&text) {
                    Ok(json) => {
                        debug!("✅ [ws_to_broadcast] JSON 解析成功: {:?}", json);
                        json
                    }
                    Err(e) => {
                        error!("❌ [ws_to_broadcast] JSON 解析失败: {} - 错误: {}", text, e);
                        continue;
                    }
                };

                let player_id = match json["player_id"].as_i64() {
                    Some(player_id) => {
                        let id = player_id as i32;
                        debug!("✅ [ws_to_broadcast] 提取 player_id: {}", id);
                        id
                    }
                    None => {
                        error!("❌ [ws_to_broadcast] player_id字段不存在: {}", text);
                        continue;
                    }
                };

                let content = match json["content"].as_str() {
                    Some(content) => {
                        debug!("✅ [ws_to_broadcast] 提取 content: {}", content);
                        content.to_string()
                    }
                    None => {
                        error!("❌ [ws_to_broadcast] content字段不存在: {}", text);
                        continue;
                    }
                };

                let mes_type = match json["mes_type"].as_str() {
                    Some(mes_type) => {
                        debug!("✅ [ws_to_broadcast] 提取 type: {}", mes_type);
                        mes_type.to_string()
                    }
                    None => {
                        error!("❌ [ws_to_broadcast] type字段不存在: {}", text);
                        continue;
                    }
                };
                if mes_type == "text" {
                    match tx.send(MessageType::Text(MessageResponse {
                        player_id,
                        content: content.clone(),
                    })) {
                        Ok(_) => {
                            debug!("✅ [ws_to_broadcast] 消息广播成功");
                        }
                        Err(e) => {
                            error!("❌ [ws_to_broadcast] 消息广播失败: {} - 错误: {}", text, e);
                            continue;
                        }
                    };
                } else if mes_type == "emoji" {
                    match tx.send(MessageType::Emoji(MessageResponse { player_id, content })) {
                        Ok(_) => {
                            debug!("✅ [ws_to_broadcast] 消息广播成功");
                        }
                        Err(e) => {
                            error!("❌ [ws_to_broadcast] 消息广播失败: {} - 错误: {}", text, e);
                            continue;
                        }
                    };
                }
            }
            Message::Close(close_frame) => {
                debug!("📨 [ws_to_broadcast] 收到关闭消息: {:?}", close_frame);
                if room_id == player_id {
                    let player_ids: Vec<i32> = {
                        let room_info = match (*state).room_info.get(&room_id) {
                            Some(room) => room,
                            None => {
                                error!("❌ [ws_to_broadcast] 房间不存在");
                                continue;
                            }
                        };

                        room_info.players.iter().map(|p| p.player_id).collect()
                    };

                    for pid in player_ids {
                        if pid != player_id {
                            (*state).normal_quit_room.insert(pid, ());
                        }
                        match tx.send(MessageType::Quit(pid, room_id)) {
                            Ok(_) => {
                                debug!(
                                    "✅ [ws_to_broadcast] 退出消息广播成功 - player_id: {}",
                                    pid
                                );
                            }
                            Err(e) => {
                                error!("❌ [ws_to_broadcast] 退出消息广播失败: 错误: {e}");
                            }
                        }
                    }
                } else {
                    match tx.send(MessageType::Quit(player_id, room_id)) {
                        Ok(_) => {
                            debug!(
                                "✅ [ws_to_broadcast] 退出消息广播成功 - player_id: {}",
                                player_id
                            );
                        }
                        Err(e) => {
                            error!("❌ [ws_to_broadcast] 退出消息广播失败: 错误: {e}");
                        }
                    }
                }
                break;
            }
            Message::Binary(binary) => {
                debug!("📨 [ws_to_broadcast] 收到二进制消息: {:?}", binary);
                continue;
            }
            Message::Ping(ping) => {
                // debug!("📨 [ws_to_broadcast] 收到 Ping 消息: {:?}", ping);
                continue;
            }
            Message::Pong(pong) => {
                (*state).last_pong.insert(player_id, Instant::now());
                // debug!("📨 [ws_to_broadcast] 收到 Pong 消息: {:?}", pong);
                continue;
            }
            _ => {
                debug!("📨 [ws_to_broadcast] 收到未知消息: {:?}", msg);
                continue;
            }
        };
    }

    debug!("🛑 [ws_to_broadcast] WebSocket 接收任务结束");
}

/// 处理从广播通道接收的消息并发送到 WebSocket
pub async fn handle_broadcast_to_ws(
    mut ws_sink: Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>,
    tx: tokio::sync::broadcast::Sender<MessageType>,
    player: Player,
    content: String,
    state: AppState,
) {
    debug!("🚀 [broadcast_to_ws] 启动广播监听任务");

    // 通知所有用户同步状态
    debug!(
        "📢 [broadcast_to_ws] 准备发送登录通知 - player_id: {}, content: {}",
        player.player_id, content
    );

    debug!("🔄 [broadcast_to_ws] 开始订阅广播频道");
    let mut rx = tx.subscribe();

    loop {
        debug!("⏳ [broadcast_to_ws] 等待接收广播消息...");
        match rx.recv().await {
            Ok(data) => {
                match data {
                    MessageType::Text(MessageResponse { player_id, content }) => {
                        debug!(
                            "📨 [broadcast_to_ws] 收到广播消息: player_id={}, content={}",
                            player_id, content
                        );
                        let json_msg = json!({
                            "type": "text",
                            "player_id": player_id,
                            "content": content,
                        });
                        debug!(
                            "📤 [broadcast_to_ws] 准备发送消息到 WebSocket: {:?}",
                            json_msg
                        );

                        if let Err(e) = (*ws_sink)
                            .lock()
                            .await
                            .send(Message::Text(json_msg.to_string().into()))
                            .await
                        {
                            error!("❌ [broadcast_to_ws] WebSocket 发送消息失败 - 错误: {}", e);
                        } else {
                            debug!("✅ [broadcast_to_ws] 消息发送成功");
                        }
                    }
                    MessageType::Emoji(MessageResponse { player_id, content }) => {
                        let json_msg = json!({
                            "type": "emoji",
                            "player_id": player_id,
                            "content": content,
                        });
                        debug!(
                            "📤 [broadcast_to_ws] 准备发送消息到 WebSocket: {:?}",
                            json_msg
                        );
                        if let Err(e) = ws_sink
                            .lock()
                            .await
                            .send(Message::Text(json_msg.to_string().into()))
                            .await
                        {
                            error!("❌ [broadcast_to_ws] WebSocket 发送消息失败 - 错误: {}", e);
                        } else {
                            debug!("✅ [broadcast_to_ws] 消息发送成功");
                        }
                    }
                    MessageType::Sync(room_info) => {
                        debug!("同步状态");
                        let json_msg = json!({
                            "type": "sync",
                            "room_info": room_info,
                        });
                        debug!(
                            "📤 [broadcast_to_ws] 准备发送消息到 WebSocket: {:?}",
                            json_msg
                        );
                        if let Err(e) = ws_sink
                            .lock()
                            .await
                            .send(Message::Text(json_msg.to_string().into()))
                            .await
                        {
                            error!("❌ [broadcast_to_ws] WebSocket 发送消息失败 - 错误: {}", e);
                        } else {
                            debug!("✅ [broadcast_to_ws] 消息发送成功");
                        }
                        drop(room_info);
                    }
                    MessageType::Quit(quit_player_id, room_id) => {
                        debug!("🛑 [broadcast_to_ws] 收到退出消息");
                        debug!(
                            "quit_player_id :{quit_player_id} palyer_id :{},room_id :{room_id}",
                            player.player_id
                        );
                        if quit_player_id == player.player_id {
                            debug!("🛑 [broadcast_to_ws] 自己退出房间");
                            let room_info = match state.inner.room_info.get(&room_id) {
                                Some(room) => room,
                                None => {
                                    error!("❌ [broadcast_to_ws] 房间不存在");
                                    // continue;
                                }
                            };

                            let room_info_clone = room_info.clone();
                            drop(room_info);
                            match tx.send(MessageType::Sync(room_info_clone)) {
                                Ok(_) => {
                                    debug!("✅ [broadcast_to_ws] 同步消息广播成功");
                                    if (*state).normal_quit_room.get(&quit_player_id).is_some() {
                                        let close_frame =
                                            Message::Close(Some(axum::extract::ws::CloseFrame {
                                                code: 1000, // 正常关闭
                                                reason: "User quit".into(),
                                            }));
                                        match ws_sink.lock().await.send(close_frame).await {
                                            Ok(_) => {
                                                info!("✅ [broadcast_to_ws] 关闭帧发送成功");
                                            }
                                            Err(e) => {
                                                error!(
                                                    "❌ [broadcast_to_ws] quit_player_id :{quit_player_id} 关闭帧发送失败: 错误: {e}"
                                                );
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("❌ [broadcast_to_ws] 同步消息广播失败 - 错误: {}", e);
                                }
                            };
                            break;
                        } else {
                            debug!("🛑 [broadcast_to_ws] 其他玩家退出房间");
                            continue;
                        }
                    }
                };
            }
            Err(e) => {
                error!("❌ [broadcast_to_ws] 接收广播消息时发生错误: {}", e);
                if let tokio::sync::broadcast::error::RecvError::Closed = e {
                    debug!("🔒 [broadcast_to_ws] 广播通道已关闭");
                    let close_frame = Message::Close(Some(axum::extract::ws::CloseFrame {
                        code: 1008,
                        reason: "inactivetimeout".into(),
                    }));
                    if ws_sink.lock().await.send(close_frame).await.is_err() {
                        error!("❌ [broadcast_to_ws] 关闭帧发送失败");
                    }
                    break;
                }
            }
        }
    }

    debug!("🛑 [broadcast_to_ws] 广播监听任务结束");
}

// 心跳任务
async fn heartbeat_task(
    mut ws_sink: Arc<Mutex<futures::stream::SplitSink<WebSocket, Message>>>,
    player_id: i32,
    state: AppState,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;

        // 检查上次收到 Pong 的时间
        let last_pong = (*state)
            .last_pong
            .entry(player_id)
            .or_insert(Instant::now());
        let elapsed = last_pong.elapsed();

        if elapsed > tokio::time::Duration::from_secs(10) {
            // 90秒内没收到 Pong，认为连接已死
            error!("💔 [heartbeat] 90秒内未收到 Pong，连接可能已断开");
            break;
        }

        // debug!("💓 [heartbeat] 发送 Ping (上次 Pong: {:?}秒前)", elapsed.as_secs());

        if let Err(e) = ws_sink
            .lock()
            .await
            .send(Message::Ping(Bytes::from_static(b"ping")))
            .await
        {
            error!("❌ [heartbeat] Ping 发送失败: {}", e);
            break;
        }
        drop(last_pong);
    }
}
