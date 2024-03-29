use tokio::io::AsyncWriteExt;

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*, utils::kick::kick};
use crate::game::{blocks::*, world::*};

pub async fn handle_pdig<'a>(packet: &Packet<'a>, pc: &mut PlayerConection) -> Result<(), Box<dyn Error>> {
    let s = packet.content[0];
    let x = i32::from_be_bytes(packet.content[1..=4].try_into()?);
    let y = packet.content[5];
    let z = i32::from_be_bytes(packet.content[6..=9].try_into()?);

    match s {
        0 | 2 => { // todo: ignore 0 if is in survival
            *get_world().block_at(BlockPosition { x, y, z }) = Block::default();
            
            let mut packet = Packet::new(0x35);
            packet.append(&x.to_be_bytes());
            packet.append(&y.to_be_bytes());
            packet.append(&z.to_be_bytes());
            packet.append(&[0, 0]);

            unsafe {
                for (_, pc) in &mut *PLAYER_CONS {
                    let socket =get_stream(pc.id);
                    socket.write_all(&packet.to_vec()).await?;
                }
            }
        },
        _ => kick(pc, &format!("0E s {s:02x}")).await?
    }

    Ok(())
}
