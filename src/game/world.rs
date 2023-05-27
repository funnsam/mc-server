use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::{game::{chunks::*, entity::*}, protocol::utils::send_chunk::send_chunk};

use std::{collections::HashMap, error::Error, sync::*};

lazy_static::lazy_static! {
    static ref WORLD: World = World::new();
}

pub fn get_world() -> &'static mut World {
    unsafe { &mut *(&*WORLD as *const World as *mut World) }
}

#[derive(Clone)]
pub struct World {
    pub chunks: HashMap<ChunkPosition, Chunk>,
    pub entities: HashMap<i32, Entity>
}

unsafe impl Send for World {}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entities: HashMap::new()
        }
    }

    pub fn spawn_entity(&mut self, e: Entity) -> i32 {
        self.entities.insert(0, e);
        0
    }

    pub fn chunk_at(&mut self, p: ChunkPosition) -> &Chunk {
        if self.chunks.contains_key(&p) {
            &self.chunks[&p]
        } else {
            self.chunks.insert(p, Chunk::default());
            &self.chunks[&p]
        }
    }
    
    pub async fn send_chunks(&mut self, s: &mut TcpStream, nc: ChunkPosition, oc: Option<ChunkPosition>) -> Result<(), Box<dyn Error>> {
        let r = crate::config::CONFIG.general.view_distance;
        for x in -r..r {
            for z in -r..r {
                let t = ChunkPosition {
                    x: nc.x + x,
                    z: nc.z + z
                };
                
                if oc.is_some() {
                    if  (nc.x - oc.unwrap().x).abs() > r ||
                        (nc.z - oc.unwrap().z).abs() > r {
                        continue;
                    }
                }

                send_chunk(s, self.chunk_at(t), t).await?;
            }
        }
        Ok(())
    }
}
