use tokio::io::AsyncWriteExt;

use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};
use crate::game::{world::*, entity::*};

pub async fn handle_pmove<'a, 'b>(packet: &Packet<'a>, pc: &mut PlayerConection<'b>) -> Result<(), Box<dyn Error>> {
    let x = f64::from_be_bytes(packet.content[0..=7].try_into().unwrap());
    let y = f64::from_be_bytes(packet.content[8..=15].try_into().unwrap());
    let z = f64::from_be_bytes(packet.content[24..=31].try_into().unwrap());

    get_world().entities.get_mut(&0).unwrap().position = EntityPosition {x, y, z};

    Ok(())
}
