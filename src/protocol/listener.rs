use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::error::Error;

use crate::protocol::handlers::*;
use crate::protocol::{packet::Packet, player_cons::*};

pub async fn run() -> Result<(), Box<dyn Error>> {
    println!("\x1b[1;34minfo: \x1b[0mStarting server");

    let listener = TcpListener::bind("127.0.0.1:65535").await?;
    loop {
        println!("w");
        let (mut socket, _) = listener.accept().await?;
        println!("g");

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let n = socket
                    .read(&mut buf)
                    .await
                    .unwrap();
            if n == 0 { return; }

            let packet = Packet::from_slice(&buf[0..n]);

            match buf[0] {
                0x02 => {
                    let socket_id = add_stream(socket);
                    let mut socket = get_stream(socket_id);
                    handle_handshake(&packet, &mut socket, socket_id).await.unwrap();
                },
                0xFE => { handle_ping(&packet, &mut socket).await.unwrap(); },
                _ => println!("\x1b[1;33mwarning: \x1b[0mReceived unknown packet kind: {:02x}, content: {:?}", packet.kind, packet.content),
            }
        });
    }
}
