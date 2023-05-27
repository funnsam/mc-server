use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::{game::{chunks::*, entities::*, blocks::*}, protocol::utils::chunk_ops::*};

use std::{collections::HashMap, error::Error};

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

    pub fn chunk_at(&mut self, p: ChunkPosition) -> &mut Chunk {
        if !self.chunks.contains_key(&p) {
            self.chunks.insert(p, Chunk::default());
        }
        self.chunks.get_mut(&p).unwrap()
    }

    pub fn block_at(&mut self, p: BlockPosition) -> &mut Block {
        let c = self.chunk_at(ChunkPosition { x: p.x >> 4, z: p.z >> 4 });
        &mut c.blocks[(p.x & 15) as usize][p.y as usize][(p.z & 15) as usize]
    }
    
    pub async fn send_chunks(&mut self, s: &mut TcpStream, nc: ChunkPosition, oc: Option<ChunkPosition>) -> Result<(), Box<dyn Error>> {
        let r = crate::config::CONFIG.general.view_distance;
        let oc = oc.unwrap_or(ChunkPosition { x: r << 1, z: r << 1 });
        for x in -r..r {
            for z in -r..r {
                let np = ChunkPosition {
                    x: nc.x + x,
                    z: nc.z + z
                };
                let op = ChunkPosition {
                    x: oc.x + x,
                    z: oc.z + z
                };
               
                // chunk loading
                if  (oc.x - np.x).abs() > r ||
                    (oc.z - np.z).abs() > r {
                    send_chunk(s, self.chunk_at(np), np).await?
                }

                // chunk unloading
                if  (nc.x - op.x).abs() > r ||
                    (nc.z - op.z).abs() > r {
                    request_unload_chunk(s, op).await?;
                }
            }
        }
        Ok(())
    }
}
