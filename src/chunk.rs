mod common;
mod post_flattening;
mod pre_flattening;
mod palette;

use std::str::FromStr;
use std::time::SystemTime;

use crate::biome::Biome;
use crate::block::Block;
use crate::block_cuboid::BlockCuboid;
use crate::block_entity::BlockEntity;
use crate::coordinates::ChunkCoord;
use crate::height_map::HeightMap;
use crate::light_cuboid::LightCuboid;
use crate::mc_version::{McVersion, THE_FLATTENING};
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
    pub fn raw_chunk_zlib(&self) -> RawChunkData {
        // Time of update is now
        let last_update = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // TODO: Biomes currently hard coded to plains.
        // TODO: Add biomes for post flattening saves.
        let biomes: Vec<u8> = match &self.biomes {
            Some(biomes) => biomes.iter().map(|biome| u8::from(*biome)).collect(),
            None => vec![Biome::Plains.into(); 256],
        };
        let biomes = utils::vec_u8_into_vec_i8(biomes);

        // Various calculations
        let sections = if self.data_version < THE_FLATTENING {
            self.pre_flattening_sections()
        } else {
            self.post_flattening_sections()
        };
        let tile_entities = self.pre_flattening_tile_entities();

        // Prior to 21w43a, most tags were inside a tag named "Level".
        // From 21w43a onwards, those tags are directly in the base tag.
        // We still create the level tag here, and optionally fills it (or not) depending on
        // version.
        let mut level: nbt::Map<String, nbt::Value> = nbt::Map::with_capacity(13);
        let mut nbt = nbt::Blob::new();

        // Prepare values to go either in "Level" or base tag.
        let x_pos = nbt::Value::Int(self.global_pos.0 as i32);
        let z_pos = nbt::Value::Int(self.global_pos.1 as i32);
        let last_update = nbt::Value::Long(last_update as i64);
        let inhabited_time = nbt::Value::Long(0); // TODO implement

        // Put values in correct tag.
        if self.data_version < McVersion::from_str("21w43a").unwrap() {
            // Fill the Level compund tag (before 21w43a)
            level.insert("xPos".into(), x_pos);
            level.insert("zPos".into(), z_pos);
            level.insert("LastUpdate".into(), last_update);
            level.insert("InhabitedTime".into(), inhabited_time);
        } else {
            // Fill the base nbt structure (21w43a or later)
            nbt.insert("xPos", x_pos).unwrap();
            nbt.insert("zPos", z_pos).unwrap();
            nbt.insert("LastUpdate", last_update).unwrap();
            nbt.insert("InhabitedTime", inhabited_time).unwrap();
        }

        if self.data_version < THE_FLATTENING {
            // Legacy format
            level.insert("TerrainPopulated".into(), nbt::Value::Byte(1));
            level.insert("LightPopulated".into(), nbt::Value::Byte(1));
            level.insert("V".into(), nbt::Value::Byte(1));
            level.insert("Biomes".into(), nbt::Value::ByteArray(biomes));
            level.insert(
                "HeightMap".into(),
                nbt::Value::IntArray(self.height_map().into()),
            );
            level.insert("Sections".into(), sections);
        } else if self.data_version < McVersion::from_str("21w43a").unwrap() {
            // Fill the level compound tag
            level.insert("Status".into(), nbt::Value::String("full".into()));
            level.insert("isLightOn".into(), nbt::Value::Byte(0));
            level.insert("Sections".into(), sections);
        } else {
            // Fill the base nbt structure
            nbt.insert("Status", nbt::Value::String("full".into())).unwrap();
            nbt.insert("isLightOn", nbt::Value::Byte(0)).unwrap();
            nbt.insert("sections", sections).unwrap();
        }

        // TODO Add proper handling of entities, instead of forgetting them
        let entities = nbt::Value::List(Vec::<nbt::Value>::new());

        if self.data_version < McVersion::from_str("21w43a").unwrap() {
            level.insert("Entities".into(), entities);
            level.insert("TileEntities".into(), tile_entities);
        } else {
            nbt.insert("Entities", entities).unwrap();
            nbt.insert("block_entities", tile_entities).unwrap();
        }
        // TODO Also insert "TileTicks" (optional)

        nbt.insert("DataVersion", self.data_version.id()).unwrap();
        if self.data_version < McVersion::from_str("21w43a").unwrap() {
            nbt.insert("Level", nbt::Value::Compound(level)).unwrap();
        }

        RawChunkData::new_zlib(&nbt)
    }

    /// Creates a chunk from raw chunk (NBT) data.
    pub fn from_raw_chunk_data(data: &RawChunkData) -> Self {
        let nbt = data.to_nbt();

        const THE_FLATTENING: &str = "17w47a";
        let data_version = nbt_blob_lookup_int(&nbt, "DataVersion")
            .map(McVersion::from_id)
            .unwrap();

        let x_pos = nbt_blob_lookup_int(
            &nbt,
            if data_version < McVersion::from_str("21w43a").unwrap() {
                "Level/xPos"
            } else {
                "xPos"
            },
        ).unwrap();

        let z_pos = nbt_blob_lookup_int(
            &nbt,
            if data_version < McVersion::from_str("21w43a").unwrap() {
                "Level/zPos"
            } else {
                "zPos"
            },
        ).unwrap();

        let global_pos: ChunkCoord = (x_pos.into(), z_pos.into()).into();

        let _last_update = nbt_blob_lookup_long(
            &nbt,
            if data_version < McVersion::from_str("21w43a").unwrap() {
                "Level/LastUpdate"
            } else {
                "LastUpdate"
            },
        ).unwrap();

        let tile_entities = nbt_blob_lookup(
            &nbt,
            if data_version < McVersion::from_str("21w43a").unwrap() {
                "Level/TileEntities"
            } else {
                "block_entities"
            },
        ).unwrap_or_else(|err| panic!("Level/TileEntities not found: {}", err));

        let mut block_entities = BlockEntity::map_from_nbt_list(&tile_entities, data_version);

        let biomes: Option<Vec<Biome>> = nbt_blob_lookup_byte_array(&nbt, "Level/Biomes")
            .map(|biomes| biomes.iter().map(|biome| Biome::from(*biome as u8)).collect::<Vec<Biome>>()).ok();

        let sections = nbt_blob_lookup_list(
            &nbt,
            if data_version < McVersion::from_str("21w43a").unwrap() {
                "Level/Sections"
            } else {
                "sections"
            },
        ).unwrap_or_else(|err| panic!("Level/Sections not found: {}", err));

        // TODO import height maps
        /*
        let height_map = nbt_blob_lookup(&nbt, "Level/HeightMap")
            .unwrap_or_else(|| panic!("Level/HeightMap not found"));
        println!("Height map: {:#?}", height_map);
        */

        // Fist pass: Prepare pseudo bock entities for block data that is stored
        // in one block but used for another. This may cross section boundaries.
        if data_version < McVersion::from_str(THE_FLATTENING).unwrap() {
            for section in &sections {
                block_entities.extend(
                    Chunk::pre_flattening_pseudo_block_entities(section, &global_pos).into_iter(),
                );
            }
        }

        // Second pass: Collect the full set of (finished) blocks
        // TODO chunk height increased at some point and must be handled correctly here
        let mut block_cuboid = BlockCuboid::new_filled((16, 256, 16), Block::Air);
        
        if data_version < McVersion::from_str(THE_FLATTENING).unwrap() {
            for section in &sections {
                Chunk::pre_flattening_fill_block_cuboid_from_section(
                    section,
                    &block_entities,
                    &global_pos,
                    &mut block_cuboid,
                );
            }
        } else {
            for section in &sections {
                Chunk::post_flattening_fill_block_cuboid_from_section(
                    data_version,
                    section,
                    &block_entities,
                    &global_pos,
                    &mut block_cuboid,
                );
            }
        }

        // Get block light and sky light data out from the sections
        let mut block_light = LightCuboid::new((16, 256, 16));
        let mut sky_light = LightCuboid::new((16, 256, 16));

        for section in &sections {
            Chunk::pre_flattening_fill_light_cuboids_from_section(
                section,
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
