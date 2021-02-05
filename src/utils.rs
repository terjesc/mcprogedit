//! General utility functions for internal use throughout mcprogedit.

/// Get the nibble at nibble position `index`.
pub(crate) fn nibble(vec: &[u8], index: usize) -> u8 {
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
pub(crate) fn vec_i8_into_vec_u8(mut vec: Vec<i8>) -> Vec<u8> {
    let p = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
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
