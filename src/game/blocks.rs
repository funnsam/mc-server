#[derive(Clone, Default)]
pub struct Block {
    pub id: u8,
    pub meta: u8
}

#[derive(Clone, Default, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: u8,
    pub z: i32,
}
