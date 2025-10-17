/*
 * @Author: error: error: git config user.name & please set dead value or install git && error: git config user.email & please set dead value or install git & please set dead value or install git
 * @Date: 2025-04-16 14:46:58
 * @LastEditors: error: error: git config user.name & please set dead value or install git && error: git config user.email & please set dead value or install git & please set dead value or install git
 * @LastEditTime: 2025-07-25 19:50:57
 * @FilePath: /oxide/src/lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use anyhow::Result;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::ops::Deref;
pub mod config;
pub use config::*;
pub mod handlers;
pub use handlers::*;
pub mod dto;
pub use dto::*;
pub mod types;
pub use types::*;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;



pub fn get_route(state: AppState) -> Router {
    // 创建 CORS 中间件
    let cors = CorsLayer::new()
        // 允许所有来源（开发环境用，生产环境应该限制为特定域名）
        // 明确允许前端开发服务器的源
        .allow_origin(tower_http::cors::Any)
        // 允许标准的 HTTP 方法
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::OPTIONS,
            http::Method::DELETE,
            http::Method::PUT,
        ])
        // 允许标准 HTTP 头部
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::CONTENT_LENGTH,      // 文件上传需要
            http::header::CONTENT_DISPOSITION, // 文件上传需要
            http::header::CONTENT_ENCODING,
        ]);
    
    Router::new()
        .route("/ws", get(handlers::websocket_handler))
        .route("/create", post(create_room))
        .layer(cors)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<InnerAppState>,
}
impl AppState {
    pub fn try_new(_config: &Config) -> Result<Self> {
        Ok(AppState {
            inner: Arc::new(InnerAppState::new()),
        })
    }
}

impl Deref for AppState {
    type Target = InnerAppState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


#[derive(Clone)]
// 服务器状态
pub struct InnerAppState {
    // 活跃会话
    pub room_broadcast_couple: Arc<DashMap<i32, room_broadcast_couple>>,
    pub room_info: Arc<DashMap<i32, Room>>,
    // pub pool: PgPool,
    // // 用于数据数据解密
    // pub public_key: Vec<u8>,
    // // 用于数据加密
    // pub private_key: Vec<u8>,
}

impl InnerAppState {
    pub(crate) fn new() -> Self {
        InnerAppState {
            room_broadcast_couple: Arc::new(DashMap::new()),
            room_info: Arc::new(DashMap::new()),
        }
    }
}

/// SSH会话中的消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 普通数据 (0)
    Data = 0,
    /// 超时消息 (1)
    Timeout = 1,
    /// 断开连接消息 (2)
    Disconnect = 2,
    /// 错误消息 (3)
    Error = 3,
    /// 调整终端大小 (4)
    Resize = 4,
}

impl MessageType {
    /// 从字节转换为消息类型
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(MessageType::Data),
            1 => Some(MessageType::Timeout),
            2 => Some(MessageType::Disconnect),
            3 => Some(MessageType::Error),
            4 => Some(MessageType::Resize),
            _ => None, // 无效的消息类型
        }
    }

    /// 转换为字节
    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

/// SSH会话消息结构
#[derive(Debug, Clone)]
pub struct SessionMessage {
    /// 消息类型
    pub message_type: MessageType,
    /// 消息内容
    pub content: Vec<u8>,
}

impl SessionMessage {
    /// 创建新的会话消息
    pub fn new(message_type: MessageType, content: Vec<u8>) -> Self {
        Self {
            message_type,
            content,
        }
    }

    /// 创建数据消息
    pub fn data(content: Vec<u8>) -> Self {
        Self::new(MessageType::Data, content)
    }

    /// 创建超时消息
    pub fn timeout(message: &str) -> Self {
        Self::new(MessageType::Timeout, message.as_bytes().to_vec())
    }

    /// 创建断开连接消息
    pub fn disconnect(message: &str) -> Self {
        Self::new(MessageType::Disconnect, message.as_bytes().to_vec())
    }

    /// 创建错误消息
    pub fn error(message: &str) -> Self {
        Self::new(MessageType::Error, message.as_bytes().to_vec())
    }

    /// 从字节序列解析消息
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let message_type = MessageType::from_byte(bytes[0])?;
        let content = bytes[1..].to_vec();

        Some(Self {
            message_type,
            content,
        })
    }

    /// 序列化为字节序列
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.message_type.to_byte()];
        bytes.extend_from_slice(&self.content);
        bytes
    }
}

