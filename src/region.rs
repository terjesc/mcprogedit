use byteorder::{BigEndian, ReadBytesExt};

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::SystemTime;

use crate::coordinates::ChunkCoord;

const SECTOR_LEN_BYTES: usize = 4096;
const SECTOR_LEN_WORDS: usize = SECTOR_LEN_BYTES / 4;

pub struct Region {
    chunks: HashMap<ChunkCoord, ChunkData>,
}

impl Region {
    pub fn load_from_file(region_file_path: &std::path::Path) -> Self {
        //  Region file format:
        //      | byte  | 0 - 4095  | 4096 - 8181 | 8192 -     |
        //      +-------+-----------+-------------+------------+
        //      | field | locations | timestamps  | chunk data |
        //
        //  "locations" and "timestamps" are 1024 element arrays, each with one entry
        //      for each of the region's chunks. Entry for chunk at chunk coordinates
        //      (x, z) is found at byte offset ``4 * ((x & 31) + (z & 31) * 32)´´.
        //
        //  location: Where in the file to find "chunk data".
        //
        //      | byte  | 0 1 2  | 3            |
        //      +-------+--------+--------------+
        //      | field | offset | sector count |
        //
        //  offset: big-endian offset in 4 KiB sectors from the start of the file,
        //      for where the chunk data starts.
        //
        //  sector count: length of chunk data, in 4 KiB sectors (rounded up)
        //  * max chunk size is 1 MiB
        //  * Special value 0x00000000 means there is no chunk data for the given chunk
        //
        //  timestamp: 4 byte big endian unix epoch timestamp,
        //      when the chunk was last modified

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

                let data_len_bytes = location.sector_count as usize * SECTOR_LEN_BYTES;
                let data_offset_bytes = location.offset as usize * SECTOR_LEN_BYTES;

                region_file
                    .seek(SeekFrom::Start(data_offset_bytes as u64))
                    .unwrap_or_else(|_| panic!("Could not seek to {}", data_offset_bytes));
                let mut data = Vec::<u8>::with_capacity(data_len_bytes);
                let _ = region_file
                    .by_ref()
                    .take(data_len_bytes as u64)
                    .read_to_end(&mut data);

                chunks.insert((x as i64, z as i64).into(), ChunkData { timestamp, data });
            }
        }

        Self { chunks }
    }

    pub fn save_to_file(&self, _region_file_path: &std::path::Path) {
        unimplemented!();
    }

    pub fn get_chunk_data(&self, local_chunk_coordinates: &ChunkCoord) -> Option<&Vec<u8>> {
        match self.chunks.get(local_chunk_coordinates) {
            Some(chunk) => Some(&chunk.data),
            None => None,
        }
    }

    pub fn set_chunk_data(&mut self, local_chunk_coordinates: &ChunkCoord, data: Vec<u8>) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        self.chunks
            .insert(*local_chunk_coordinates, ChunkData { timestamp, data });
    }
}

struct ChunkData {
    timestamp: u32,
    data: Vec<u8>,
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
