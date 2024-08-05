use rkyv::{Archive, Deserialize, Serialize};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_LENGTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub enum BlockId {
    Air,
    Grass,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
#[archive(check_bytes)]
#[archive_attr(derive(Debug))]
pub struct Chunk {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub blocks: [BlockId; CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT],
}
