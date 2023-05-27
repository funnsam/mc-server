use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::utils::*;
use crate::protocol::packet::Packet;
use crate::config::*;

pub async fn handle_ping<'a>(_packet: &Packet<'a>, socket: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut ret_pack = Packet::new(0xFF);
    ret_pack.append(&write_packet::write_str16(
        &format!("{}\u{A7}69\u{A7}420", CONFIG.server_info)
    ).unwrap());
    socket.write_all(&ret_pack.to_vec()).await?;

    Ok(())
}
