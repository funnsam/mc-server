use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::protocol::{packet::*, utils::write_packet::write_str16};

use std::error::Error;

static mut CHAT_MSGS: Vec<String> = Vec::new();

pub fn push_chat(s: String) {
    unsafe { CHAT_MSGS.push(s) }
}

pub async fn sync() -> Result<(), Box<dyn Error>> { unsafe {
    let mut pks = Vec::with_capacity(CHAT_MSGS.len());
    for i in &CHAT_MSGS {
        let mut packet = Packet::new(0x03);
        packet.append(&write_str16(i).unwrap());
        pks.push(packet.to_vec());

        println!("\x1b[1;36mingame: \x1b[0m{}", i);
    }

    for pc in &mut crate::protocol::player_cons::PLAYER_CONS {
        let socket = &mut pc.socket;
        for i in &pks {
            socket.write_all(&i).await?;
        }
    }

    CHAT_MSGS.clear();

    Ok(())
}}
