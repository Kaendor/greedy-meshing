use crate::CHUNK_SIZE_CUBED;

pub struct Chunk {
    pub voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let voxels = (0..CHUNK_SIZE_CUBED)
            .map(|_p| Voxel {
                kind: BlockKind::Rock,
            })
            .collect();
        Chunk { voxels }
    }
}

pub struct Voxel {
    kind: BlockKind,
}

pub enum BlockKind {
    Rock,
    Air,
}
