use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use lazy_mut::*;
use crate::protocol::handlers::*;
use crate::protocol::{packet::Packet, utils::read_packet::*};

use std::error::Error;
use std::collections::HashMap;

pub struct PlayerConection {
    pub id: usize,
    pub username: String
}

impl PlayerConection {
    pub fn new(s: usize, u: String) -> Self {
        PlayerConection {
            id: s,
            username: u
        }
    }
}

lazy_mut! {
    pub static mut PLAYER_CONS: HashMap<usize, PlayerConection> = HashMap::new();
    pub static mut PLAYER_STRS: HashMap<usize, TcpStream> = HashMap::new();
    pub static mut PLAYER_AT: usize = 0;
}

pub fn add_stream(s: TcpStream) -> usize {
    unsafe {
        PLAYER_STRS.insert(*PLAYER_AT, s);
        *PLAYER_AT += 1;
        *PLAYER_AT - 1
    }
}

pub fn get_stream(id: usize) -> &'static mut TcpStream {
    unsafe {
        PLAYER_STRS.get_mut(&id).unwrap()
    }
}

pub fn add_player(s: usize, u: String) {
    unsafe {
        PLAYER_CONS.insert(s, PlayerConection::new(s, u));
    }
}

pub fn remove_player(u: &String) { unsafe {
    let mut indx = None;
    for (i, pc) in PLAYER_CONS.iter().enumerate() {
        if pc.1.username == *u {
            indx = Some(i);
            break;
        }
    }
    if indx.is_some() {
        let i = indx.unwrap();
        PLAYER_CONS.remove(&i);
        PLAYER_STRS.remove(&i);
    }
}}


pub async fn listen_players() -> Result<(), Box<dyn Error>> { unsafe {
    println!("\x1b[1;34minfo: \x1b[0mStarting b");

    loop {
        for (i, pc) in PLAYER_CONS.iter_mut() {
            let socket = &mut get_stream(pc.id);

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
                    PLAYER_CONS.remove(&i);
                    PLAYER_STRS.remove(&i);
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
        for (_, pc) in PLAYER_CONS.iter_mut() {
            let mut p = Packet::new(0x00);
            p.append(&0_u32.to_be_bytes());
            get_stream(pc.id).write_all(&p.to_vec()).await?;
        }
        std::thread::sleep_ms(3000);
    }
}}
