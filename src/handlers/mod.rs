
mod websocket;
pub use websocket::*;
pub mod room;
pub use room::*;
pub mod car;
pub use car::*;
pub mod friend;
pub use friend::*;
pub mod player;
pub use player::*;



pub async fn index() -> &'static str {
    "Hello, world!"
}
