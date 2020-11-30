
pub enum RawChunkData {
    Empty,
    GZip(Vec<u8>),
    ZLib(Vec<u8>),
    Uncompressed(Vec<u8>),
}


//  chunk data format:
//      | byte  | 0 1 2 3 | 4           | 5 -  |
//      +-------+---------+-------------+------+
//      | field | length  | compression | data |
//
//  length: length in bytes, of compression type + data
//
//  compression:
//      | bit   | 7          | 6 - 0            |
//      +-------+------------+------------------+
//      | field | chunk file | compression type |
//
//  chunk file:
//      if 1, data is stored in a file with name on the format c.x.z.mcc,
//      where x and z are the chunk coordinates, instead of in the data field.
//      This may happen for chunks that do not fit within 1 MiB.
//      (The field is marked "since a version before 1.15.1" on the wiki.)
//
//  compression type: compression used on the "data" field. Default value 2.
//
//      | value | method          |
//      +-------+-----------------+
//      | 1     | GZip (RFC 1952) |
//      | 2     | Zlib (RFC 1950) | <-- always used for writing by the official client
//      | 3     | no compression  |
//
//  data: len - 1 bytes of chunk data in NBT format,
//      compressed using method indicated by "compression type"
