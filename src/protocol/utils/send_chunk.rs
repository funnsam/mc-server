use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::{game::chunks::*, protocol::packet::*};

use std::io::Write;

use deflate::Compression;
use deflate::write::ZlibEncoder;

pub async fn send_chunk(socket: &mut TcpStream, c: &Chunk, p: ChunkPosition) -> Result<(), Box<dyn std::error::Error>> {
    let mut ret_pack = Packet::new(0x32);
    ret_pack.append(&p.x.to_be_bytes());
    ret_pack.append(&p.z.to_be_bytes());
    ret_pack.append(&1_i8.to_be_bytes());
    socket.write_all(&ret_pack.to_vec()).await?;

    let rawd = c.to_sendable();
    let cd = deflate(&rawd);

    let mut ret_pack = Packet::new(0x33);
    ret_pack.append(&(p.x << 4).to_be_bytes());
    ret_pack.append(&0_i16.to_be_bytes());
    ret_pack.append(&(p.z << 4).to_be_bytes());
    ret_pack.append(&[15, 127, 15]);
    ret_pack.append(&(cd.len() as i32).to_be_bytes());
    ret_pack.append(&cd);
    socket.write_all(&ret_pack.to_vec()).await?;

    Ok(())
}

fn deflate(rd: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write_all(rd).unwrap();
    encoder.finish().unwrap()
}
