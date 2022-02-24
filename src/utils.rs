//! General utility functions for internal use throughout mcprogedit.

/// Get the nibble at nibble position `index`.
pub(crate) fn _nibble(vec: &[u8], index: usize) -> u8 {
    let byte_index = index / 2;
    if index % 2 == 0 {
        vec[byte_index] & 0x0F
    } else {
        (vec[byte_index] & 0xF0) >> 4
    }
}

/// Put the four lowest bits of `nibble` into the nibble position `index`.
pub(crate) fn set_nibble(vec: &mut [u8], nibble: u8, index: usize) {
    let byte_index = index / 2;
    if index % 2 == 0 {
        // least significant nibble
        vec[byte_index] = (vec[byte_index] & 0xF0) | (nibble & 0x0F);
    } else {
        // most significant nibble
        vec[byte_index] = (vec[byte_index] & 0x0F) | ((nibble << 4) & 0xF0);
    };
}

/// Convert Vec<i8> into Vec<u8>. Useful for converting Hematite NBT byte arrays,
/// which come as Vec<i8>, into the more handy Vec<u8> format (for bit
/// manipulation, etc.)
pub(crate) fn _vec_i8_into_vec_u8(mut vec: Vec<i8>) -> Vec<u8> {
    let p = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

/// Convert Vec<i32> into Vec<u32>. Useful for converting Hematite NBT int arrays,
/// which come as Vec<i32>, into the more handy Vec<u32> format (for bit
/// manipulation, etc.)
pub(crate) fn vec_i32_into_vec_u32(mut vec: Vec<i32>) -> Vec<u32> {
    let p = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(p as *mut u32, len, cap) }
}

/// Convert Vec<u8> into Vec<i8>. Useful for making and manipulating byte arrays,
/// that are later to be added to an NBT tag, as the former format is best suited
/// for bit manipulation, while the latter format is the format required by the
/// Hematite NBT library.
pub(crate) fn vec_u8_into_vec_i8(mut vec: Vec<u8>) -> Vec<i8> {
    let p = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

/// Convert Vec<u32> into Vec<i32>. Useful for making and manipulating int arrays,
/// that are later to be added to an NBT tag, as the former format is best suited
/// for bit manipulation, while the latter format is the format required by the
/// Hematite NBT library.
pub(crate) fn _vec_u32_into_vec_i32(mut vec: Vec<u32>) -> Vec<i32> {
    let p = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(p as *mut i32, len, cap) }
}

/// Packs an array into a tightly packed array of u64.
///
/// This is the value packing used between the flattening and Minecraft 1.16. Values of length
/// bits_per_value are placed back to back in the memory space of the output array.
pub(crate) fn tightly_packed<T>(unpacked_array: &[T], bits_per_value: usize) -> Vec<u64> where
    u64: From<T>,
    T: Copy,
{
    let unpacked_len = unpacked_array.len();
    let packed_len_bits = bits_per_value * unpacked_len;
    let packed_len = (packed_len_bits + 64 - 1) / 64;
    let mut packed = Vec::with_capacity(packed_len);

    let value_mask = (1 << bits_per_value) - 1;
    let values_overlapping_u64_max = ((64 - 2) / bits_per_value) + 2;

    for long_index in 0 .. packed_len {
        let mut long = 0u64;
        let packed_long_bit_index = long_index * 64;
        let low_unpacked_index = (packed_long_bit_index + bits_per_value - 1) / bits_per_value;
        let high_unpacked_index = std::cmp::min(unpacked_len, low_unpacked_index + values_overlapping_u64_max);

        #[allow(clippy::needless_range_loop)]
        for unpacked_index in low_unpacked_index .. high_unpacked_index {
            // The values with these indexes are possibly overlapping with the given element of the
            // packed array
            let value: u64 = Into::<u64>::into(unpacked_array[unpacked_index]) & value_mask;
            let value_bit_index = bits_per_value * unpacked_index;
            
            if value_bit_index < packed_long_bit_index {
                long |= value.checked_shr((packed_long_bit_index - value_bit_index) as u32).unwrap_or(0);
            } else {
                long |= value.checked_shl((value_bit_index - packed_long_bit_index) as u32).unwrap_or(0);
            }
        }

        packed.push(long);
    }
    packed
}

/// Unpacks a tightly packed array stored in a u64 array.
///
/// This is the value unpacking used between the flattening and Minecraft 1.16. Values of length
/// bits_per_value are placed back to back in the memory space of the output array.
pub(crate) fn tightly_unpacked<T>(packed_array: &[u64], bits_per_value: usize) -> Vec<T> where
    T: std::convert::TryFrom<u64>,
    <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
{
    let packed_len = packed_array.len();
    let unpacked_len = packed_len * 64 / bits_per_value;
    let mut unpacked = Vec::<T>::with_capacity(unpacked_len);

    let value_mask = (1 << bits_per_value) - 1;

    for unpacked_index in 0 .. unpacked_len {
        let value_bit_index = unpacked_index * bits_per_value;
        let long_index = value_bit_index / 64;
        let internal_bit_index = value_bit_index % 64;

        let mut value = (packed_array[long_index] >> internal_bit_index) & value_mask;
        if (64 - internal_bit_index) < bits_per_value {
            value |= (packed_array[long_index + 1] << (64 - internal_bit_index)) & value_mask;
        }
        unpacked.push(std::convert::TryInto::<T>::try_into(value).unwrap());
    }

    unpacked
}

/// Packs an array into an array of padded u64.
///
/// This is the value packing used for Minecraft 1.16 and later. Values of length bits_per_value
/// are placed back to back in each u64, but padding is added in the most significant end of the
/// u64 when there is not enough space left for storing a full value.
pub(crate) fn paddedly_packed<T>(unpacked_array: &[T], bits_per_value: usize) -> Vec<u64> where
    u64: From<T>,
    T: Copy,
{
    let full_values_per_u64 = 64 / bits_per_value;
    let unpacked_len = unpacked_array.len();
    let packed_len = (unpacked_len + full_values_per_u64 - 1) / full_values_per_u64;
    let mut packed = Vec::with_capacity(packed_len);

    let value_mask = (1 << bits_per_value) - 1;

    for long_index in 0 .. packed_len {
        let mut long = 0u64;
        for internal_index in 0 .. full_values_per_u64 {
            let unpacked_index = long_index * full_values_per_u64 + internal_index;
            if unpacked_index >= unpacked_len {
                break;
            }
            long |= (Into::<u64>::into(unpacked_array[unpacked_index]) & value_mask) << (internal_index * bits_per_value);
        }
        packed.push(long);
    }
    packed
}

/// Unpacks a padded array stored in a u64 array.
///
/// This is the value unpacking used for Minecraft 1.16 and later. Values of length bits_per_value
/// are placed back to back in each u64, but padding is added in the most significant end of the
/// u64 when there is not enough space left for storing a full value.
pub(crate) fn paddedly_unpacked<T>(packed_array: &[u64], bits_per_value: usize) -> Vec<T> where
    T: std::convert::TryFrom<u64>,
    <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
{
    let full_values_per_u64 = 64 / bits_per_value;
    let packed_len = packed_array.len();
    let unpacked_len = packed_len * full_values_per_u64;
    let mut unpacked = Vec::with_capacity(unpacked_len);

    let value_mask = (1 << bits_per_value) - 1;

    for long in packed_array {
        for internal_index in 0 .. full_values_per_u64 {
            let value = (long >> (internal_index * bits_per_value)) & value_mask;
            unpacked.push(std::convert::TryInto::<T>::try_into(value).unwrap());
        }
    }

    unpacked
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector of packed nibbles into byte vector
/// The packing is little endian
pub(crate) fn packed_nibbles_to_bytes(nibbles: &[i8]) -> Vec<i8> {
    nibbles
        .iter()
        .flat_map(|byte| vec![byte & 0x0F, (byte >> 4) & 0x0F])
        .collect()
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector into byte vector of packed nibbles
/// The packing is little endian
pub(crate) fn _bytes_to_packed_nibbles(bytes: &[i8]) -> Vec<i8> {
    bytes
        .chunks(2)
        .map(|c| c.iter().fold(0i8, |acc, x| (acc >> 4) + ((x & 0x0F) << 4)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNPACKED_U8: [u8; 26] = [1,2,2,3,4,4,5,6,6,4,8,0,7,4,3,13,15,16,9,14,10,12,0,2,11,4];
    const TIGHTLY_PACKED_5: [u64; 3] = [0x7020863148418841, 0x8B1018A7260F68C8, 0x0000000000000000];
    const PADDEDLY_PACKED_5: [u64; 3] = [0x0020863148418841, 0x01018A7260F68C87, 0x000000000000008B];

    #[test]
    fn test_tight_packing() {
        assert_eq!(TIGHTLY_PACKED_5, tightly_packed(&UNPACKED_U8, 5).as_slice());
    }

    #[test]
    fn test_tight_unpacking() {
        assert_eq!(UNPACKED_U8, tightly_unpacked(&TIGHTLY_PACKED_5, 5).as_slice()[..26]);
    }

    #[test]
    fn test_padded_packing() {
        assert_eq!(PADDEDLY_PACKED_5, paddedly_packed(&UNPACKED_U8, 5).as_slice());
    }

    #[test]
    fn test_padded_unpacking() {
        assert_eq!(UNPACKED_U8, paddedly_unpacked(&PADDEDLY_PACKED_5, 5).as_slice()[..26]);
    }

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
            _bytes_to_packed_nibbles(&[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            vec![0x10, 0x32, 0x54, 0x76]
        );
    }
}
