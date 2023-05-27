use super::blocks::Block;

#[derive(Clone, Default, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

#[derive(Clone)]
pub struct Chunk {
    pub blocks: Vec<Vec<Vec<Block>>>
}

impl Default for Chunk {
    fn default() -> Self {
        let mut s = Self::new();

        for x in 0..16 {
            for z in 0..16 {
                s.blocks[x][0][z].id = 7;
                for y in 1..61 {
                    s.blocks[x][y][z].id = 1;
                }
                for y in 61..63 {
                    s.blocks[x][y][z].id = 3;
                }
                s.blocks[x][63][z].id = 2;
            }
        }

        s
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![vec![Block::default(); 16]; 128]; 16]
        }
    }

    pub fn to_sendable(&self) -> Vec<u8> {
        let mut ba = Vec::with_capacity(16*16*128);
        let mut bb = Vec::with_capacity((16*16*128)/2);
        let mut bc = Vec::with_capacity((16*16*128)/2);
        let mut bd = Vec::with_capacity((16*16*128)/2);

        for x in 0..16 {
            for z in 0..16 {
                for y in 0..128 {
                    ba.push(self.blocks[x][y][z].id);
                    if y & 1 == 0 {
                        bb.push(self.blocks[x][y][z].meta << 4);
                        bc.push(15 << 4);
                        bd.push(15 << 4);
                    } else {
                        *bb.last_mut().unwrap() |= self.blocks[x][y][z].meta;
                        *bc.last_mut().unwrap() |= 15;
                        *bd.last_mut().unwrap() |= 15;
                    }
                }
            }
        }

        [ba, bb, bc, bd].concat()
    }
}
