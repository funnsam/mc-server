use std::error::Error;

use crate::protocol::{packet::Packet, player_cons::*};
use crate::game::world::*;

pub async fn handle_cia<'a>(packet: &Packet<'a>, pc: &mut PlayerConection) -> Result<(), Box<dyn Error>> {
    get_world().entities.get_mut(&0).unwrap().do_action(0, packet.content);

    Ok(())
}
