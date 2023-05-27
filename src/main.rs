use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

pub mod game;
pub mod protocol;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let a = tokio::spawn(async move {
        protocol::listener::run().await.unwrap();
    });
    let b = tokio::spawn(async move {
        protocol::player_cons::listen_players().await.unwrap();
    });
    let c = tokio::spawn(async move {
        protocol::player_cons::keep_alives().await.unwrap();
    });
    let d = tokio::spawn(async move {
        loop {
            game::on_tick::on_tick().await.unwrap();
            std::thread::sleep_ms(50);
        }
    });


    a.await?;
    b.await?;
    c.await?;
    d.await?;

    Ok(())
}
