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
                println!("Has GZip data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_gzip_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::ZLib(chunk_data) => {
                println!("Has ZLib data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_zlib_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Uncompressed(chunk_data) => {
                println!("Has uncompressed data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Empty => nbt::Blob::new(),
        }
    }
}

pub struct Chunk {
    data_version: McVersion,
    global_pos: ChunkCoord,
    last_update: i64,
    //biome: BiomeMapping,
    //entities: HashMap<BlockCoord, Vec<Entity>>,
    //blocks: ???<Block>, // ???
}

impl Chunk {
    pub fn from_raw_chunk_data(data: &RawChunkData) -> Self {
        let nbt = data.to_nbt();
        //println!("{}", nbt);

        let data_version = nbt_blob_lookup_int(&nbt, "DataVersion")
            .map(McVersion::from_id)
            .unwrap();

        let x_pos = nbt_blob_lookup_int(&nbt, "Level/xPos").unwrap();
        let z_pos = nbt_blob_lookup_int(&nbt, "Level/zPos").unwrap();
        let global_pos: ChunkCoord = (x_pos.into(), z_pos.into()).into();

        let last_update = nbt_blob_lookup_long(&nbt, "Level/LastUpdate").unwrap();

        let tile_entities = nbt_blob_lookup(&nbt, "Level/TileEntities")
            .unwrap_or_else(|| panic!("Level/TileEntities not found"));
        let block_entities = BlockEntity::map_from_nbt_list(&tile_entities);
        //println!("TileEntities: {:#?}", block_entities);

        let sections = nbt_blob_lookup_list(&nbt, "Level/Sections")
            .unwrap_or_else(|| panic!("Level/Sections not found"));
        for section in sections {
            println!("Y index: {:?}", nbt_value_lookup(&section, "Y"));
        }

        Self {
            data_version,
            global_pos,
            last_update,
        }
    }
}
