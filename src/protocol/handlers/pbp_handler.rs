use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*, utils::kick::kick};

pub async fn handle_pbp<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let mut x = i32::from_be_bytes(packet.content[0..=3].try_into()?);
    let mut y = packet.content[4];
    let mut z = i32::from_be_bytes(packet.content[5..=8].try_into()?);
    let d = packet.content[9];
    let mut bid = i16::from_be_bytes(packet.content[10..=11].try_into()?);
    let mut qty = 0;
    let mut dmg = 0;

    if bid.is_negative() {
        bid = bid.abs();
        qty = packet.content[12];
        dmg = i16::from_be_bytes(packet.content[13..=14].try_into().unwrap());
    }

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

        let mut packet = Packet::new(0x35);
        packet.append(&x.to_be_bytes());
        packet.append(&y.to_be_bytes());
        packet.append(&z.to_be_bytes());
        packet.append(&[bid as u8, dmg as u8]);

        unsafe {
            for pc in &mut PLAYER_CONS {
                let socket = &mut pc.socket;
                socket.write_all(&packet.to_vec()).await?;
            }
        }
    }

    Ok(())
}
