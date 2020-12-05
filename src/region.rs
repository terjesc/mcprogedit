use byteorder::{BigEndian, ReadBytesExt};

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::SystemTime;

use crate::chunk::RawChunkData;
use crate::coordinates::ChunkCoord;

const SECTOR_LEN_BYTES: usize = 4096;
const SECTOR_LEN_WORDS: usize = SECTOR_LEN_BYTES / 4;

pub struct Region {
    chunks: HashMap<ChunkCoord, InternalChunkData>,
}

impl Region {
    pub fn load_from_file(region_file_path: &std::path::Path) -> Self {
        let mut region_file = File::open(region_file_path)
            .unwrap_or_else(|_| panic!("Unable to open region file {:?}", region_file_path));

        let mut locations = Vec::<Location>::with_capacity(1024);
        for _ in 0..1024 {
            locations.push(Location::new(
                region_file.read_u24::<BigEndian>().unwrap(),
                region_file.read_u8().unwrap(),
            ));
        }

        let mut timestamps = Vec::<u32>::with_capacity(1024);
        for _ in 0..1024 {
            timestamps.push(region_file.read_u32::<BigEndian>().unwrap());
        }

        let mut chunk_data = Vec::<u8>::with_capacity(0);
        let _ = region_file.read_to_end(&mut chunk_data);

        let mut chunks = HashMap::with_capacity(1024);
        for x in 0..32 {
            for z in 0..32 {
                let index = x + z * 32;
                let timestamp = timestamps[index];
                let location = locations[index];

                // If the location field is all 0, then there is no chunk.
                if location.offset == 0 {
                    continue;
                }

                let data_offset_bytes = location.offset as usize * SECTOR_LEN_BYTES;

                region_file
                    .seek(SeekFrom::Start(data_offset_bytes as u64))
                    .unwrap_or_else(|_| panic!("Could not seek to {}", data_offset_bytes));

                // Read length and prepare data buffer
                let chunk_len = region_file.read_u32::<BigEndian>().unwrap();
                let mut chunk_data: Vec<u8> = Vec::with_capacity((chunk_len - 1) as usize);

                // Read compression
                let chunk_compression = region_file.read_u8().unwrap();

                // check bit 7
                if (chunk_compression & 0x80) == 0x80 {
                    todo!("Import data from separate chunk file!");
                } else if let Ok(chunk_file) = region_file.try_clone() {
                    chunk_file
                        .take((chunk_len - 1) as u64)
                        .read_to_end(&mut chunk_data)
                        .unwrap();
                }

                // Deliver data in the correct compression type
                let data = match chunk_compression & 3 {
                    0x01 => RawChunkData::GZip(chunk_data),
                    0x02 => RawChunkData::ZLib(chunk_data),
                    0x03 => RawChunkData::Uncompressed(chunk_data),
                    _ => panic!("Unknown compression format: {}", chunk_compression & 0x03),
                };

                chunks.insert(
                    (x as i64, z as i64).into(),
                    InternalChunkData { timestamp, data },
                );
            }
        }

        Self { chunks }
    }

    pub fn save_to_file(&self, _region_file_path: &std::path::Path) {
        unimplemented!();
    }

    pub fn get_chunk_data(&self, local_chunk_coordinates: &ChunkCoord) -> RawChunkData {
        match self.chunks.get(local_chunk_coordinates) {
            Some(chunk) => chunk.data.clone(),
            None => RawChunkData::Empty,
        }
    }

    pub fn set_chunk_data(&mut self, local_chunk_coordinates: &ChunkCoord, data: RawChunkData) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        self.chunks.insert(
            *local_chunk_coordinates,
            InternalChunkData { timestamp, data },
        );
    }
}

struct InternalChunkData {
    timestamp: u32,
    data: RawChunkData,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Location {
    offset: u32,
    sector_count: u8,
}

impl Location {
    fn new(offset: u32, sector_count: u8) -> Self {
        Self {
            offset,
            sector_count,
        }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("L({}:{})", self.offset, self.sector_count))
    }
}
