use tokio::io::AsyncWriteExt;

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*, utils::kick::kick};
use crate::game::{world::*, blocks::*};

pub async fn handle_pbp<'a>(packet: &Packet<'a>, pc: &mut PlayerConection) -> Result<(), Box<dyn Error>> {
    let mut x = i32::from_be_bytes(packet.content[0..=3].try_into()?);
    let mut y = packet.content[4];
    let mut z = i32::from_be_bytes(packet.content[5..=8].try_into()?);
    let d = packet.content[9];

    if x == -1 && y == 0xFF && z == -1 && d == 0xFF {
        kick(pc, "0F xyzd -1").await?;
    } else {
        match d {
            0 => y -= 1,
            1 => y += 1,
            2 => z -= 1,
            3 => z += 1,
            4 => x -= 1,
            5 => x += 1,
            _ => kick(pc, &format!("0F d {d:02x}")).await?
        }
        
        let hbindx = i16::from_be_bytes(get_world().entities.get_mut(&0).unwrap().do_action(3, &[]).unwrap().try_into().unwrap()) + 36;
        let hitem  = get_world().entities.get_mut(&0).unwrap().do_action(1, &hbindx.to_be_bytes()).unwrap();
        let id = i16::from_be_bytes(hitem[0..=1].try_into().unwrap()) as u8;
        let meta = i16::from_be_bytes(hitem[2..=3].try_into().unwrap()) as u8;

        *get_world().block_at(BlockPosition { x, y, z }) = Block { id, meta };

        let mut packet = Packet::new(0x35);
        packet.append(&x.to_be_bytes());
        packet.append(&y.to_be_bytes());
        packet.append(&z.to_be_bytes());
        packet.append(&[id, meta]);

        unsafe {
            for (_, pc) in &mut *PLAYER_CONS {
                let socket = get_stream(pc.id);
                socket.write_all(&packet.to_vec()).await?;
            }
        }
    }

    Ok(())
}
