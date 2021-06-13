mod pre_flattening;

use std::str::FromStr;
use std::time::SystemTime;

use crate::biome::Biome;
use crate::block::Block;
use crate::block_cuboid::BlockCuboid;
use crate::block_entity::BlockEntity;
use crate::coordinates::ChunkCoord;
use crate::height_map::HeightMap;
use crate::light_cuboid::LightCuboid;
use crate::mc_version::McVersion;
use crate::nbt_lookup::*;
use crate::utils;

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

    fn new_zlib(nbt: &nbt::Blob) -> Self {
        let mut chunk_data: Vec<u8> = Vec::new();
        nbt.to_zlib_writer(&mut chunk_data)
            .unwrap_or_else(|err| panic!("Bad chunk write: {}", err));
        Self::ZLib(chunk_data)
    }
}

pub struct Chunk {
    data_version: McVersion,
    global_pos: ChunkCoord,
    _last_update: i64,
    //entities: HashMap<BlockCoord, Vec<Entity>>,
    pub(crate) blocks: BlockCuboid,
    pub(crate) block_light: LightCuboid,
    pub(crate) sky_light: LightCuboid,
    biomes: Option<Vec<Biome>>,
}

impl Chunk {
    pub fn new(chunk_position: ChunkCoord) -> Self {
        Chunk {
            data_version: McVersion::from_str("1.12.2").unwrap(),
            global_pos: chunk_position,
            _last_update: 0,
            blocks: BlockCuboid::new((16, 256, 16)),
            block_light: LightCuboid::new((16, 256, 16)),
            sky_light: LightCuboid::new((16, 256, 16)),
            biomes: None,
        }
    }

    pub fn chunk_coordinates(&self) -> &ChunkCoord {
        &self.global_pos
    }

    /// Generates Zlib compressed raw chunk data from the chunk object.
    // NB only pre-flattening chunk saving as of yet
    pub fn raw_chunk_zlib(&self) -> RawChunkData {
        // Time of update is now
        let last_update = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Biomes needs some extra handling...
        let biomes: Vec<u8> = match &self.biomes {
            Some(biomes) => biomes.iter().map(|biome| u8::from(*biome)).collect(),
            None => vec![Biome::Plains.into(); 256],
        };
        let biomes = utils::vec_u8_into_vec_i8(biomes);

        // Various calculations
        let sections = self.pre_flattening_sections();
        let tile_entities = self.pre_flattening_tile_entities();

        // Create the Level compund tag
        let mut level: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(13);
        level.insert("xPos".into(), nbt::Value::Int(self.global_pos.0 as i32));
        level.insert("zPos".into(), nbt::Value::Int(self.global_pos.1 as i32));
        level.insert("LastUpdate".into(), nbt::Value::Long(last_update as i64));
        level.insert("LightPopulated".into(), nbt::Value::Byte(1));
        level.insert("TerrainPopulated".into(), nbt::Value::Byte(1));
        level.insert("V".into(), nbt::Value::Byte(1));
        level.insert("InhabitedTime".into(), nbt::Value::Long(0));
        level.insert("Biomes".into(), nbt::Value::ByteArray(biomes));
        level.insert(
            "HeightMap".into(),
            nbt::Value::IntArray(self.height_map().into()),
        );
        level.insert("Sections".into(), sections);
        // TODO Add proper handling of entities, instead of forgetting them:
        level.insert(
            "Entities".into(),
            nbt::Value::List(Vec::<nbt::Value>::new()),
        );
        level.insert("TileEntities".into(), tile_entities);
        // TODO Also insert "TileTicks" (optional)

        // Create and return nbt blob
        let mut nbt = nbt::Blob::new();
        nbt.insert("DataVersion", self.data_version.id()).unwrap();
        nbt.insert("Level", nbt::Value::Compound(level)).unwrap();
        RawChunkData::new_zlib(&nbt)
    }

    /// Creates a chunk from raw chunk (NBT) data.
    // NB only pre-flattening chunk loading as of yet
    // TODO Move pre-flattening import implementation to the pre-flattening file.
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

        let biomes: Option<Vec<Biome>> = nbt_blob_lookup_byte_array(&nbt, "Level/Biomes")
            .map(|biomes| biomes.iter().map(|biome| Biome::from(*biome as u8)).collect());

        let sections = nbt_blob_lookup_list(&nbt, "Level/Sections")
            .unwrap_or_else(|| panic!("Level/Sections not found"));

        /*
        let height_map = nbt_blob_lookup(&nbt, "Level/HeightMap")
            .unwrap_or_else(|| panic!("Level/HeightMap not found"));
        println!("Height map: {:#?}", height_map);
        */

        // Fist pass: Prepare pseudo bock entities for block data that is stored
        // in one block but used for another. This may cross section boundaries.
        for section in &sections {
            block_entities.extend(
                Chunk::pre_flattening_pseudo_block_entities(&section, &global_pos).into_iter(),
            );
        }

        // Second pass: Collect the full set of (finished) blocks
        let mut block_cuboid = BlockCuboid::new_filled((16, 256, 16), Block::Air);
        for section in &sections {
            // TODO rename to pre_flattening_fill_block_cuboid_from_section
            Chunk::pre_flattening_section_into_block_cuboid(
                &section,
                &block_entities,
                &global_pos,
                &mut block_cuboid,
            );
        }

        // Get block light and sky light data out from the sections
        let mut block_light = LightCuboid::new((16, 256, 16));
        let mut sky_light = LightCuboid::new((16, 256, 16));

        for section in &sections {
            Chunk::pre_flattening_fill_light_cuboids_from_section(
                &section,
                &mut block_light,
                &mut sky_light,
            );
        }

        // Return chunk
        Self {
            data_version,
            global_pos,
            _last_update,
            blocks: block_cuboid,
            block_light,
            sky_light,
            biomes,
        }
    }

    fn height_map(&self) -> HeightMap {
        self.blocks.height_map()
    }
}
