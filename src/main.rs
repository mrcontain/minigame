use anyhow::Result;
use minigame::*;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling;
use tracing_subscriber::{fmt::Layer, prelude::*};
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::DEBUG);

    //output the error logs to the error.log
    // and only the info logs and warn logs to the oxideleap.log
    let filter_appender = rolling::daily("logs", "oxideleap.log");
    let (non_blocking, _guard1) = tracing_appender::non_blocking(filter_appender);
    let filter_layer = Layer::new()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);
    let error_appender = rolling::daily("logs", "error.log");
    let (non_blocking, _guard2) = tracing_appender::non_blocking(error_appender);
    let error_layer = Layer::new()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::ERROR);

    tracing_subscriber::registry()
        .with(layer)
        .with(filter_layer)
        .with(error_layer)
        .init();
    // 加载配置
    let config = match Config::try_load() {
        Ok(config) => {
            tracing::info!("✅ 配置文件加载成功");
            config
        }
        Err(e) => {
            tracing::error!("❌ 加载配置失败: {}", e);
            panic!("加载配置失败: {}", e);
        }
    };

    // 创建应用状态
    let state = match AppState::try_new(&config) {
        Ok(state) => {
            tracing::info!("✅ 应用状态初始化成功");
            state
        }
        Err(e) => {
            tracing::error!("❌ 创建应用状态失败: {}", e);
            panic!("创建应用状态失败: {}", e);
        }
    };

    let app = get_route(state);

    // get ip and port from config
    let addr = format!("{}:{}", config.get_bind_address(), config.get_bind_port());

    tracing::info!("🚀 正在启动服务器...");
    tracing::info!("📡 监听地址: {}", addr);
    tracing::info!("📝 WebSocket 路由: ws://{}/ws", addr);
    tracing::info!("📝 创建房间路由: http://{}/create", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("✅ 服务器启动成功！");

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}