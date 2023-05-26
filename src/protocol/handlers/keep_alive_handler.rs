use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};

pub async fn handle_keep_alive<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let socket = &mut pc.socket;
    socket.write_all(&packet.to_vec()).await?;

    Ok(())
}
