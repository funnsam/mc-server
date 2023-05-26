use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::protocol::{packet::*, utils::write_packet::write_str16};

use std::error::Error;

pub static mut TAB_ITEMS: Vec<String> = Vec::new();

pub async fn sync() -> Result<(), Box<dyn Error>> { unsafe {
    let mut pks = Vec::with_capacity(TAB_ITEMS.len());
    for i in &TAB_ITEMS {
        let mut packet = Packet::new(0xC9);
        packet.append(&write_str16(i).unwrap());
        packet.append(&1_u8.to_be_bytes());
        packet.append(&0_i16.to_be_bytes());
        pks.push(packet.to_vec());
    }

    for pc in &mut crate::protocol::player_cons::PLAYER_CONS {
        let socket = &mut pc.socket;
        for i in &pks {
            socket.write_all(&i).await?;
        }
    }

    Ok(())
}}
