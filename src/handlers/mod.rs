
mod websocket;
pub use websocket::*;
pub mod room;
pub use room::*;


const PAGE_SIZE: i64 = 10;

pub async fn index() -> &'static str {
    "Hello, world!"
}
