use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::protocol::handlers::*;
use crate::protocol::{packet::Packet, utils::read_packet::*};

use std::error::Error;

pub struct PlayerConection<'a> {
    pub socket: &'a mut TcpStream,
    pub username: String
}

impl<'a> PlayerConection<'a> {
    pub fn new(s: &'a mut TcpStream, u: String) -> Self {
        PlayerConection {
            socket: s,
            username: u
        }
    }
}

pub static mut PLAYER_CONS: Vec<PlayerConection> = Vec::new();
pub static mut PLAYER_STRS: Vec<TcpStream> = Vec::new();

pub fn add_stream(s: TcpStream) -> usize {
    unsafe {
        PLAYER_STRS.push(s);
        PLAYER_STRS.len() - 1
    }
}

pub fn get_stream(id: usize) -> &'static mut TcpStream {
    unsafe {
        &mut PLAYER_STRS[id]
    }
}

pub fn add_player(s: &'static mut TcpStream, u: String) {
    unsafe {
        PLAYER_CONS.push(PlayerConection::new(s, u));
    }
}

pub fn remove_player(u: &String) { unsafe {
    let mut indx = None;
    for (i, pc) in PLAYER_CONS.iter().enumerate() {
        if pc.username == *u {
            indx = Some(i);
            break;
        }
    }
    if indx.is_some() {
        let i = indx.unwrap();
        PLAYER_CONS.remove(i);
        PLAYER_STRS.remove(i);
    }
}}


pub async fn listen_players() -> Result<(), Box<dyn Error>> { unsafe {
    println!("\x1b[1;34minfo: \x1b[0mStarting b");

    loop {
        for (i, pc) in PLAYER_CONS.iter_mut().enumerate() {
            let socket = &mut pc.socket;

            let mut buf = vec![0; 1024];
            let n = socket
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");
            if n == 0 { continue; }

            let packet = Packet::from_slice(&buf[0..n]);

            match packet.kind {
                0x00 => handle_keep_alive(&packet, pc).await.unwrap(),
                0x01 => handle_login(&packet, pc).await.unwrap(),
                0x03 => handle_chat(&packet, pc).await.unwrap(),
                0x0B => handle_pmove(&packet, pc).await.unwrap(),
                0x0A ..= 0x0D => (),
                0x0E => handle_pdig(&packet, pc).await.unwrap(),
                0x0F => handle_pbp(&packet, pc).await.unwrap(),
                0x10 => handle_hc(&packet, pc).await.unwrap(),
                0x6B => handle_cia(&packet, pc).await.unwrap(),
                0xFF => {
                    println!("\x1b[1;34minfo: \x1b[0mClient left with message: {}", read_str16(packet.content, 256).unwrap());
                    PLAYER_CONS.remove(i);
                    PLAYER_STRS.remove(i);
                },
                _ => {
                    println!("\x1b[1;33mwarning: \x1b[0mReceived unknown packet kind: {:02x}, content: {:?}", packet.kind, packet.content);
                },
            }
        }
    }
}}

pub async fn keep_alives() -> Result<(), Box<dyn Error>> { unsafe {
    loop {
        for pc in PLAYER_CONS.iter_mut() {
            let mut p = Packet::new(0x00);
            p.append(&0_u32.to_be_bytes());
            pc.socket.write_all(&p.to_vec()).await?;
        }
        std::thread::sleep_ms(3000);
    }
}}
