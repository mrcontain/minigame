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
use sqlx::PgPool;
use tracing::info;
use tracing::error;
use std::sync::Arc;
use std::ops::Deref;
use std::time::Instant;
pub mod config;
pub use config::*;
pub mod handlers;
pub use handlers::*;
pub mod dto;
pub use dto::*;
pub mod types;
pub use types::*;
pub mod models;
pub use models::*;

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
        .route("/createroom", post(create_room))
        .route("/quitroom", post(quit_room))
        .route("/changecar",post(change_car))
        .route("/changecarskin",post(change_car_skin))
        .route("/addfriend",post(add_friend))
        .route("/removefriend",post(remove_friend))
        .route("/getfriends",post(get_friends))
        .route("/addplayer",post(add_player))
        .layer(cors)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<InnerAppState>,
}
impl AppState {
    pub fn try_new(config: &Config) -> Result<Self> {
        Ok(AppState {
            inner: Arc::new(InnerAppState::new(
            &config.database,
            )),
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
    pub normal_quit_room: Arc<DashMap<i32, ()>>, // 正常退出房间
    pub last_pong:Arc<DashMap<i32, Instant>>,
    pub pool: PgPool,
    // // 用于数据数据解密
    // pub public_key: Vec<u8>,
    // // 用于数据加密
    // pub private_key: Vec<u8>,
}

impl InnerAppState {
    pub(crate) fn new(url: &str) -> Self {
        // 创建一个pgsql连接池
        let pool = match PgPool::connect_lazy(url) {
            Ok(pool) => {
                info!("Postgres connection pool created successfully");
                pool
            }
            Err(e) => {
                error!("Failed to create Postgres connection pool: {}", e);
                panic!("Failed to create Postgres connection pool");
            }
        };
        InnerAppState {
            room_broadcast_couple: Arc::new(DashMap::new()),
            room_info: Arc::new(DashMap::new()),
            normal_quit_room: Arc::new(DashMap::new()),
            last_pong: Arc::new(DashMap::new()),
            pool,
        }
    }
}
