use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::utils::*;
use crate::protocol::{packet::Packet, player_cons::*};

pub async fn handle_handshake<'a>(packet: &Packet<'a>, socket: &mut TcpStream, sid: usize) -> Result<(), Box<dyn Error>> {
    let username = read_packet::read_str16(packet.content, 16).unwrap();
    
    let mut ret_pack = Packet::new(0x02);
    ret_pack.append(&write_packet::write_str16("-").unwrap());
    socket.write_all(&ret_pack.to_vec()).await?;

    add_player(get_stream(sid), username.to_string());
    unsafe {
        crate::protocol::tab_list::TAB_ITEMS.push(username.to_string());
    }

    Ok(())
}
