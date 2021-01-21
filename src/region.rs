use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use std::collections::HashMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::SystemTime;

use crate::chunk::RawChunkData;
use crate::coordinates::ChunkCoord;

const SECTOR_LEN_BYTES: usize = 4096;

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
                let index = Self::index_from_chunk_coords(ChunkCoord::from((x, z)));
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

    pub fn save_to_file(&self, region_file_path: &std::path::Path) {
        let mut region_file = OpenOptions::new().write(true).create(true).open(region_file_path)
            .unwrap_or_else(|_| panic!("Unable to create region file {:?}", region_file_path));

        let mut next_chunk_offset_sections = 2;

        for (
            local_chunk_coordinates,
            InternalChunkData { timestamp, data: raw_chunk_data }
        ) in &self.chunks {
            // Calculate the index for location data and timestamp data for this chunk.
            let index = Self::index_from_chunk_coords(*local_chunk_coordinates);

            // Unwrap the internally stored chunk, or skip if empty.
            let (compression, data) = match raw_chunk_data {
                RawChunkData::Empty => continue,
                RawChunkData::GZip(data) => (1, data),
                RawChunkData::ZLib(data) => (2, data),
                RawChunkData::Uncompressed(data) => (3, data),
            };

            // Figure out the total size of the chunk, when stored in the region file.
            const CHUNK_HEADER_BYTES: usize = 5;
            let chunk_len_bytes = CHUNK_HEADER_BYTES + data.len();
            let chunk_len_sections = (chunk_len_bytes + SECTOR_LEN_BYTES - 1) / SECTOR_LEN_BYTES;

            // The location entry, to place at the beginning of the region file.
            let location = Location {
                offset: next_chunk_offset_sections,
                sector_count: chunk_len_sections as u8,
            };

            // The chunk header.
            let chunk_header = ChunkHeader {
                length: data.len() as u32 + 1,
                compression,
            };

            // Amount of padding to place after the chunk data, to fill the section.
            let padding_len_bytes = (chunk_len_sections * SECTOR_LEN_BYTES) - chunk_len_bytes;
            let padding = vec![0u8; padding_len_bytes];

            // Write the chunk header, chunk data, and padding.
            let data_offset_bytes = location.offset as usize * SECTOR_LEN_BYTES;
            region_file
                .seek(SeekFrom::Start(data_offset_bytes as u64))
                .unwrap_or_else(|_| panic!("Could not seek to chunk at {}", data_offset_bytes));
            region_file.write_u32::<BigEndian>(chunk_header.length).unwrap();
            region_file.write_u8(chunk_header.compression).unwrap();
            region_file.write_all(&data).unwrap();
            region_file.write_all(&padding).unwrap();

            // Write timestamp
            region_file.seek(SeekFrom::Start((SECTOR_LEN_BYTES + index * 4) as u64))
                .unwrap_or_else(|_| panic!("Could not seek to timestamp index {}", index));
            region_file.write_u32::<BigEndian>(*timestamp).unwrap();

            // Write location
            region_file.seek(SeekFrom::Start((index * 4) as u64))
                .unwrap_or_else(|_| panic!("Could not seek to location index {}", index));
            region_file.write_u24::<BigEndian>(location.offset).unwrap();
            region_file.write_u8(location.sector_count).unwrap();

            // Update chunk offset, for next chunk to be written directly after this one.
            next_chunk_offset_sections += chunk_len_sections as u32;
        }
    }

    pub fn chunk_data(&self, local_chunk_coordinates: &ChunkCoord) -> RawChunkData {
        match self.chunks.get(local_chunk_coordinates) {
            Some(chunk) => chunk.data.clone(),
            None => RawChunkData::Empty,
        }
    }

    pub fn _set_chunk_data(&mut self, local_chunk_coordinates: &ChunkCoord, data: RawChunkData) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        self.chunks.insert(
            *local_chunk_coordinates,
            InternalChunkData { timestamp, data },
        );
    }

    fn chunk_coords_from_index(index: usize) -> ChunkCoord {
        ChunkCoord::from((index as i64 % 32, index as i64 / 32))
    }

    fn index_from_chunk_coords(chunk_coordinates: ChunkCoord) -> usize {
        (chunk_coordinates.0 + 32 * chunk_coordinates.1) as usize
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct ChunkHeader {
    length: u32,
    compression: u8,
}


impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("L({}:{})", self.offset, self.sector_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coords_from_index() {
        assert_eq!(Region::chunk_coords_from_index(0), ChunkCoord::from((0, 0)));
        assert_eq!(Region::chunk_coords_from_index(31), ChunkCoord::from((31, 0)));
        assert_eq!(Region::chunk_coords_from_index(992), ChunkCoord::from((0, 31)));
        assert_eq!(Region::chunk_coords_from_index(1023), ChunkCoord::from((31, 31)));
    }

    #[test]
    fn test_index_from_chunk_coords() {
        assert_eq!(Region::index_from_chunk_coords(ChunkCoord::from((0, 0))), 0);
        assert_eq!(Region::index_from_chunk_coords(ChunkCoord::from((31, 0))), 31);
        assert_eq!(Region::index_from_chunk_coords(ChunkCoord::from((0, 31))), 992);
        assert_eq!(Region::index_from_chunk_coords(ChunkCoord::from((31, 31))), 1023);
    }

    // TODO Needs testing of writing region. Can get a half-decent test through
    // writing chunks, then use the load test on them.
    /*
    #[test]
    fn read_then_write_back_region_file() {
        let load_path = std::path::Path::new("tests/saves/1_12_2/region/r.0.0.mca");
        let save_path = std::path::Path::new("tests/output/1_12_2/region/r.0.0.mca");
        let region = Region::load_from_file(load_path);
        region.save_to_file(save_path);
    }
    */
}
