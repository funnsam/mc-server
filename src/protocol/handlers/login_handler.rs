use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use std::error::Error;

use crate::protocol::utils::*;
use crate::protocol::{packet::Packet, player_cons::*, utils::kick::kick};

pub async fn handle_login<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let cpv = i32::from_be_bytes(packet.content[0..4].try_into().unwrap());

    let socket = &mut pc.socket;

    if cpv != 17 {
        kick(pc, "Version doesn't match! Expected version MC Beta 1.8.x!").await?;
        return Ok(())
    }
    
    let mut ret_pack = Packet::new(0x01);
    ret_pack.append(&0_i32.to_be_bytes());
    ret_pack.append(&write_packet::write_str16("").unwrap());
    ret_pack.append(&0_i64.to_be_bytes());
    ret_pack.append(&1_i32.to_be_bytes());
    ret_pack.append(&0_i8.to_be_bytes());
    ret_pack.append(&0_i8.to_be_bytes());
    ret_pack.append(&128_u8.to_be_bytes());
    ret_pack.append(&3_u8.to_be_bytes());
    socket.write_all(&ret_pack.to_vec()).await?;

    let mut ret_pack = Packet::new(0x32);
    ret_pack.append(&0_i32.to_be_bytes());
    ret_pack.append(&0_i32.to_be_bytes());
    ret_pack.append(&1_i8.to_be_bytes());
    socket.write_all(&ret_pack.to_vec()).await?;

    let mut ret_pack = Packet::new(0x35);
    ret_pack.append(&0_i32.to_be_bytes());
    ret_pack.append(&64_i8.to_be_bytes());
    ret_pack.append(&0_i32.to_be_bytes());
    ret_pack.append(&2_i8.to_be_bytes());
    ret_pack.append(&0_i8.to_be_bytes());
    socket.write_all(&ret_pack.to_vec()).await?;

    let mut ret_pack = Packet::new(0x0D);
    ret_pack.append(&0.0_f64.to_be_bytes());
    ret_pack.append(&67.5_f64.to_be_bytes());
    ret_pack.append(&66.0_f64.to_be_bytes());
    ret_pack.append(&0.0_f64.to_be_bytes());
    ret_pack.append(&0.0_f32.to_be_bytes());
    ret_pack.append(&0.0_f32.to_be_bytes());
    ret_pack.append(&0_i8.to_be_bytes());
    socket.write_all(&ret_pack.to_vec()).await?;

    Ok(())
}
