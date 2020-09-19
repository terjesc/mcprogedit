//  Region file:
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
//
//  chunk data:
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
//      (The field is marked "since a version before 1.15.1 on the wiki.)
//
//  compression type: compression used on the "data" field. Default value 2.
//
//      | value | method          |
//      +-------+-----------------+
//      | 1     | GZip (RFC 1952) |
//      | 2     | Zlib (RFC 1950) |
//      | 3     | no compression  |
//
//  data: len - 1 bytes of chunk data in NBT format,
//      compressed using method indicated by "compression type"
