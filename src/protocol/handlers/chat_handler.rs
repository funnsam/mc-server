use tokio::io::AsyncWriteExt;

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*, chat::push_chat, utils::{read_packet::read_str16, write_packet::write_str16, kick::kick}};

pub async fn handle_chat<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let socket = &mut pc.socket;
    let content = read_str16(packet.content, 128)?;

    if content == "/" {
        kick(pc, "You asked for this yourself.").await?;
        return Ok(())
    }

    push_chat(format!("\u{A7}b\u{A7}E{}:\u{A7}r {}", &pc.username, content));

    Ok(())
}
