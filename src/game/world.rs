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
    
    pub async fn send_chunks(&mut self, s: &mut TcpStream, r: i32, c: ChunkPosition) -> Result<(), Box<dyn Error>> {
        for x in -r..r {
            for z in -r..r {
                let t = ChunkPosition {
                    x: c.x + x,
                    z: c.z + z
                };
                send_chunk(s, self.chunk_at(t), t).await?;
            }
        }
        Ok(())
    }
}
