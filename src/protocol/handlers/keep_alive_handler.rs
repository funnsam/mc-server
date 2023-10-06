use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};

pub async fn handle_keep_alive<'a>(packet: &Packet<'a>, pc: &mut PlayerConection) -> Result<(), Box<dyn Error>> {
    let socket = get_stream(pc.id);
    socket.write_all(&packet.to_vec()).await?;

    Ok(())
}
