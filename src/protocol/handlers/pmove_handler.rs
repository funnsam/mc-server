use tokio::io::AsyncWriteExt;

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};
use crate::game::{world::*, entities::*, chunks::*};

pub async fn handle_pmove<'a>(packet: &Packet<'a>, pc: &mut PlayerConection) -> Result<(), Box<dyn Error>> {
    let x = f64::from_be_bytes(packet.content[0..=7].try_into().unwrap());
    let y = f64::from_be_bytes(packet.content[8..=15].try_into().unwrap());
    let z = f64::from_be_bytes(packet.content[24..=31].try_into().unwrap());

    let e = get_world().entities.get_mut(&0).unwrap();

    let op = &e.position;
    let np = EntityPosition {x, y, z};

    if  (op.x as i32 >> 4 != np.x as i32 >> 4) ||
        (op.z as i32 >> 4 != np.z as i32 >> 4) {
        get_world().send_chunks(
            get_stream(pc.id),
            ChunkPosition {
                x: np.x as i32 >> 4,
                z: np.z as i32 >> 4
            },
            Some(ChunkPosition {
                x: op.x as i32 >> 4,
                z: op.z as i32 >> 4
            })
        ).await?
    }

    e.position = EntityPosition {x, y, z};

    Ok(())
}
