
mod websocket;
pub use websocket::*;
pub mod room;
pub use room::*;



pub async fn index() -> &'static str {
    "Hello, world!"
}
