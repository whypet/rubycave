pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_LENGTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub enum BlockId {
    Air,
    Grass,
}

pub struct Chunk {
    pub blocks: [BlockId; CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT],
}
