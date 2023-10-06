#![allow(cast_ref_to_mut)]

use std::error::Error;
use protocol::player_cons::*;

pub mod game;
pub mod protocol;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        PLAYER_CONS.init();
        PLAYER_STRS.init();
        PLAYER_AT.init();
    }

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
