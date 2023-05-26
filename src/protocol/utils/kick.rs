use tokio::io::AsyncWriteExt;

use crate::protocol::{packet::Packet, player_cons::*, utils::write_packet::write_str16};

pub async fn kick<'a>(pc: &mut PlayerConection<'a>, r: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ret_pack = Packet::new(0xFF);
    ret_pack.append(&write_str16(r).unwrap());
    pc.socket.write_all(&ret_pack.to_vec()).await?;
    remove_player(&pc.username);

    Ok(())
}
