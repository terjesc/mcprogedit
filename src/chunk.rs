use std::collections::HashMap;

use crate::block::Block;
use crate::block_entity::BlockEntity;
use crate::bounded_ints::*;
use crate::coordinates::{BlockCoord, ChunkCoord};
use crate::material::*;
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

        let mut blocks = Vec::new();
        let sections = nbt_blob_lookup_list(&nbt, "Level/Sections")
            .unwrap_or_else(|| panic!("Level/Sections not found"));
        for section in sections {
            println!("Y index: {:?}", nbt_value_lookup(&section, "Y"));
            blocks.extend_from_slice(
                &Chunk::section_to_block_array(&section, &block_entities)
            );
        }

        //println!("{:?}", blocks);
        let x_index = 1;
        let z_index = 1;
        let xz_index = 16 * x_index + z_index;
        let column: Vec<Block> = blocks.iter().skip(xz_index).step_by(256).cloned().collect();
        println!("{:?}", column);

        Self {
            data_version,
            global_pos,
            last_update,
        }
    }

    // TODO Move this function to a more reasonable place. A new file, perhaps?
    // It should be a non-public function. It belongs somewhat here and somewhat to Block.
    fn section_to_block_array(
        section: &nbt::Value,
        block_entities: &HashMap<BlockCoord, BlockEntity>
    ) -> Vec<Block> {
        let blocks = nbt_value_lookup_byte_array(&section, "Blocks").unwrap();
        let add = packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(&section, "Add")
                .unwrap_or_else(|| vec![0; blocks.len() / 2])
        );
        let data = packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(&section, "Data").unwrap()
        );

        blocks
            .iter()
            .enumerate()
            .map(|(index, block)| (index, ((add[index] as i16) << 8) + *block as i16))
            .map(|(index, block)| {
                match block {
                    0 => Block::Air,
                    1 => match data[index] {
                        0 => Block::Stone,
                        1 => Block::Granite,
                        2 => Block::PolishedGranite,
                        3 => Block::Diorite,
                        4 => Block::PolishedDiorite,
                        5 => Block::Andesite,
                        6 => Block::PolishedAndesite,
                        n => panic!("Unknown stone data variant: {}", n),
                    },
                    2 => Block::GrassBlock,
                    3 => match data[index] {
                        0 => Block::Dirt,
                        1 => Block::CoarseDirt,
                        2 => Block::Podzol,
                        n => panic!("Unknown dirt data variant: {}", n),
                    },
                    4 => Block::Cobblestone,
                    5 => match data[index] {
                        0 => Block::Planks { material: WoodMaterial::Oak },
                        1 => Block::Planks { material: WoodMaterial::Spruce },
                        2 => Block::Planks { material: WoodMaterial::Birch },
                        3 => Block::Planks { material: WoodMaterial::Jungle },
                        4 => Block::Planks { material: WoodMaterial::Acacia },
                        5 => Block::Planks { material: WoodMaterial::DarkOak },
                        n => panic!("Unknown plank data variant: {}", n),
                    }
                    6 => {
                        let stage = Int0Through1::new((data[index] & 0x8) >> 3).unwrap();
                        let material = match data[index] & 0x7 {
                            0 => SaplingMaterial::Oak,
                            1 => SaplingMaterial::Spruce,
                            2 => SaplingMaterial::Birch,
                            3 => SaplingMaterial::Jungle,
                            4 => SaplingMaterial::Acacia,
                            5 => SaplingMaterial::DarkOak,
                            n => panic!("Unknown sapling data variant: {}", n),
                        };
                        Block::Sapling { material, stage }
                    },
                    7 => Block::Bedrock,
                    //8 => // TODO flowing water, not yet implemented
                    9 => Block::WaterSource,
                    //10 => // TODO flowing lava, not yet implemented
                    11 => Block::LavaSource,
                    12 => match data[index] {
                        0 => Block::Sand,
                        1 => Block::RedSand,
                        n => panic!("Unknown sand data variant: {}", n),
                    }
                    13 => Block::Gravel,
                    14 => Block::GoldOre,
                    15 => Block::IronOre,
                    _ => Block::None,
                }
            })
            .collect()
    }
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector of packed nibbles into byte vector
/// The packing is little endian
fn packed_nibbles_to_bytes(nibbles: &[i8]) -> Vec<i8> {
    nibbles.iter().flat_map(|byte| vec![byte & 0x0F, byte >> 4]).collect()
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector into byte vector of packed nibbles
/// The packing is little endian
fn bytes_to_packed_nibbles(bytes: &[i8]) -> Vec<i8> {
    bytes
        .chunks(2)
        .map(|c| c.iter().fold(0i8, |acc, x| (acc >> 4) + ((x & 0x0F) << 4)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME test the full range 0-F for the nibbles.
    #[test]
    fn test_packed_nibbles_to_bytes() {
        assert_eq!(
            packed_nibbles_to_bytes(&[0x10, 0x32, 0x54, 0x76]),
            vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]
        );
    }

    // FIXME test the full range 0-F for the nibbles.
    #[test]
    fn test_bytes_to_packed_nibbles() {
        assert_eq!(
            bytes_to_packed_nibbles(&[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            vec![0x10, 0x32, 0x54, 0x76]
        );
    }
}
