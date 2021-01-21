mod pre_flattening;

use std::str::FromStr;

use crate::block_cuboid::BlockCuboid;
use crate::block_entity::BlockEntity;
use crate::coordinates::ChunkCoord;
use crate::mc_version::McVersion;
use crate::nbt_lookup::*;

#[derive(Clone)]
pub enum RawChunkData {
    Empty,
    GZip(Vec<u8>),
    ZLib(Vec<u8>),
    Uncompressed(Vec<u8>),
}

impl RawChunkData {
    fn to_nbt(&self) -> nbt::Blob {
        match self {
            RawChunkData::GZip(chunk_data) => {
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_gzip_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::ZLib(chunk_data) => {
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_zlib_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Uncompressed(chunk_data) => {
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Empty => nbt::Blob::new(),
        }
    }
}

pub struct Chunk {
    _data_version: McVersion,
    global_pos: ChunkCoord,
    _last_update: i64,
    //biome: BiomeMapping,
    //entities: HashMap<BlockCoord, Vec<Entity>>,
    blocks: BlockCuboid,
}

impl Chunk {
    pub fn blocks(&self) -> &BlockCuboid {
        &self.blocks
    }

    pub fn chunk_coordinates(&self) -> &ChunkCoord {
        &self.global_pos
    }

    // NB only pre-flattening chunk loading yet
    pub fn from_raw_chunk_data(data: &RawChunkData) -> Self {
        let nbt = data.to_nbt();

        const THE_FLATTENING: &str = "17w47a";
        let data_version = nbt_blob_lookup_int(&nbt, "DataVersion")
            .map(McVersion::from_id)
            .unwrap();
        assert!(data_version < McVersion::from_str(THE_FLATTENING).unwrap());

        let x_pos = nbt_blob_lookup_int(&nbt, "Level/xPos").unwrap();
        let z_pos = nbt_blob_lookup_int(&nbt, "Level/zPos").unwrap();
        let global_pos: ChunkCoord = (x_pos.into(), z_pos.into()).into();

        let _last_update = nbt_blob_lookup_long(&nbt, "Level/LastUpdate").unwrap();

        let tile_entities = nbt_blob_lookup(&nbt, "Level/TileEntities")
            .unwrap_or_else(|| panic!("Level/TileEntities not found"));
        let mut block_entities = BlockEntity::map_from_nbt_list(&tile_entities);

        let sections = nbt_blob_lookup_list(&nbt, "Level/Sections")
            .unwrap_or_else(|| panic!("Level/Sections not found"));

        // Fist pass: Prepare pseudo bock entities for block data that is stored
        // in one block but used for another. This may cross section boundaries.
        for section in &sections {
            block_entities.extend(Chunk::pseudo_block_entities(&section, &global_pos).into_iter());
        }

        // Second pass: Collect the full set of (finished) blocks
        let mut block_cuboid = BlockCuboid::new((16, 256, 16));
        for section in sections {
            Chunk::section_into_block_cuboid(
                &section,
                &block_entities,
                &global_pos,
                &mut block_cuboid,
            );
        }

        // Return chunk
        Self {
            _data_version: data_version,
            global_pos,
            _last_update,
            blocks: block_cuboid,
        }
    }
}
