use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};

pub async fn handle_pbp<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let mut x = i32::from_be_bytes(packet.content[0..=3].try_into()?);
    let mut y = packet.content[4];
    let mut z = i32::from_be_bytes(packet.content[5..=8].try_into()?);
    let d = packet.content[9];
    let bid = i16::from_be_bytes(packet.content[10..=11].try_into()?).abs();

    if x == -1 && y == 0xFF && z == -1 {
    } else {
        match d {
            0 => y -= 1,
            1 => y += 1,
            2 => z -= 1,
            3 => z += 1,
            4 => x -= 1,
            5 => x += 1,
            _ => return Ok(()) // assumes nothing happened
        }

        let mut packet = Packet::new(0x35);
        packet.append(&x.to_be_bytes());
        packet.append(&y.to_be_bytes());
        packet.append(&z.to_be_bytes());
        packet.append(&[bid as u8, 0]);

        unsafe {
            for pc in &mut PLAYER_CONS {
                let socket = &mut pc.socket;
                socket.write_all(&packet.to_vec()).await?;
            }
        }
    }

    Ok(())
}
