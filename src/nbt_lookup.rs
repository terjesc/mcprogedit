use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("blob does not contain any values")]
    NoValueInBlob,
    #[error("unexpected value type")]
    UnexpectedValueType,
    #[error("path not found in the NBT object")]
    PathNotFound,
}

#[allow(dead_code)]
pub fn nbt_blob_lookup(blob: &nbt::Blob, path: &'static str) -> Result<nbt::Value, Error> {
    let mut parts = path.splitn(2, '/');
    let first = parts.next().unwrap();
    let rest = parts.next().unwrap_or("");

    // Get the value (if exists)
    let value = match blob.get(first) {
        Some(value) => value,
        None => return Err(Error::NoValueInBlob),
    };

    if !rest.is_empty() {
        nbt_value_lookup(value, rest)
    } else {
        Ok(value.clone())
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_byte(blob: &nbt::Blob, path: &'static str) -> Result<i8, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Byte(byte) => Ok(byte),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_short(blob: &nbt::Blob, path: &'static str) -> Result<i16, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Short(short) => Ok(short),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_int(blob: &nbt::Blob, path: &'static str) -> Result<i32, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Int(int) => Ok(int),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_long(blob: &nbt::Blob, path: &'static str) -> Result<i64, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Long(long) => Ok(long),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_float(blob: &nbt::Blob, path: &'static str) -> Result<f32, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Float(float) => Ok(float),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_double(blob: &nbt::Blob, path: &'static str) -> Result<f64, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::Double(double) => Ok(double),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_byte_array(blob: &nbt::Blob, path: &'static str) -> Result<Vec<i8>, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::ByteArray(byte_array) => Ok(byte_array),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_string(blob: &nbt::Blob, path: &'static str) -> Result<String, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::String(string) => Ok(string),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_list(
    blob: &nbt::Blob,
    path: &'static str,
) -> Result<Vec<nbt::Value>, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::List(list) => Ok(list),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_int_array(blob: &nbt::Blob, path: &'static str) -> Result<Vec<i32>, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::IntArray(int_array) => Ok(int_array),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_long_array(blob: &nbt::Blob, path: &'static str) -> Result<Vec<i64>, Error> {
    match nbt_blob_lookup(blob, path)? {
        nbt::Value::LongArray(long_array) => Ok(long_array),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup(value: &nbt::Value, path: &'static str) -> Result<nbt::Value, Error> {
    // Split the path
    let path = path.split('/');

    let mut value = value.clone();
    // Recursively get values
    for name in path {
        if let nbt::Value::Compound(map) = value {
            match map.get(name) {
                Some(val) => value = val.clone(),
                None => return Err(Error::PathNotFound),
            }
        } else {
            return Err(Error::PathNotFound);
        }
    }
    Ok(value)
}

#[allow(dead_code)]
pub fn nbt_value_lookup_byte(value: &nbt::Value, path: &'static str) -> Result<i8, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Byte(byte) => Ok(byte),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_short(value: &nbt::Value, path: &'static str) -> Result<i16, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Short(short) => Ok(short),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_int(value: &nbt::Value, path: &'static str) -> Result<i32, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Int(int) => Ok(int),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_long(value: &nbt::Value, path: &'static str) -> Result<i64, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Long(long) => Ok(long),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_float(value: &nbt::Value, path: &'static str) -> Result<f32, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Float(float) => Ok(float),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_double(value: &nbt::Value, path: &'static str) -> Result<f64, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::Double(double) => Ok(double),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_byte_array(
    value: &nbt::Value,
    path: &'static str,
) -> Result<Vec<i8>, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::ByteArray(byte_array) => Ok(byte_array),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_string(value: &nbt::Value, path: &'static str) -> Result<String, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::String(string) => Ok(string),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_list(
    value: &nbt::Value,
    path: &'static str,
) -> Result<Vec<nbt::Value>, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::List(list) => Ok(list),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_int_array(
    value: &nbt::Value,
    path: &'static str,
) -> Result<Vec<i32>, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::IntArray(int_array) => Ok(int_array),
        _ => Err(Error::UnexpectedValueType),
    }
}

#[allow(dead_code)]
pub fn nbt_value_lookup_long_array(
    value: &nbt::Value,
    path: &'static str,
) -> Result<Vec<i64>, Error> {
    match nbt_value_lookup(value, path)? {
        nbt::Value::LongArray(long_array) => Ok(long_array),
        _ => Err(Error::UnexpectedValueType),
    }
}
