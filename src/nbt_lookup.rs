#[allow(dead_code)]
pub fn nbt_blob_lookup(blob: &nbt::Blob, path: &'static str) -> Option<nbt::Value> {
    let mut parts = path.splitn(2, '/');
    let first = parts.next().unwrap();
    let rest = parts.next().unwrap_or("");

    // Get the value (if exists)
    let value = match blob.get(first) {
        Some(value) => value,
        None => return None,
    };

    if !rest.is_empty() {
        nbt_value_lookup(value, rest)
    } else {
        Some(value.clone())
    }
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_byte(blob: &nbt::Blob, path: &'static str) -> Option<i8> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Byte(byte) => Some(byte),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_short(blob: &nbt::Blob, path: &'static str) -> Option<i16> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Short(short) => Some(short),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_int(blob: &nbt::Blob, path: &'static str) -> Option<i32> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Int(int) => Some(int),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_long(blob: &nbt::Blob, path: &'static str) -> Option<i64> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Long(long) => Some(long),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_float(blob: &nbt::Blob, path: &'static str) -> Option<f32> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Float(float) => Some(float),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_double(blob: &nbt::Blob, path: &'static str) -> Option<f64> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::Double(double) => Some(double),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_byte_array(blob: &nbt::Blob, path: &'static str) -> Option<Vec<i8>> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::ByteArray(byte_array) => Some(byte_array),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_string(blob: &nbt::Blob, path: &'static str) -> Option<String> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::String(string) => Some(string),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_list(blob: &nbt::Blob, path: &'static str) -> Option<Vec<nbt::Value>> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::List(list) => Some(list),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_int_array(blob: &nbt::Blob, path: &'static str) -> Option<Vec<i32>> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::IntArray(int_array) => Some(int_array),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_blob_lookup_long_array(blob: &nbt::Blob, path: &'static str) -> Option<Vec<i64>> {
    nbt_blob_lookup(blob, path).and_then(|value| match value {
        nbt::Value::LongArray(long_array) => Some(long_array),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup(value: &nbt::Value, path: &'static str) -> Option<nbt::Value> {
    // Split the path
    let path = path.split('/');

    let mut value = value.clone();
    // Recursively get values
    for name in path {
        if let nbt::Value::Compound(map) = value {
            match map.get(name) {
                Some(val) => value = val.clone(),
                None => return None,
            }
        } else {
            return None;
        }
    }
    Some(value)
}

#[allow(dead_code)]
pub fn nbt_value_lookup_byte(value: &nbt::Value, path: &'static str) -> Option<i8> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Byte(byte) => Some(byte),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_short(value: &nbt::Value, path: &'static str) -> Option<i16> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Short(short) => Some(short),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_int(value: &nbt::Value, path: &'static str) -> Option<i32> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Int(int) => Some(int),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_long(value: &nbt::Value, path: &'static str) -> Option<i64> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Long(long) => Some(long),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_float(value: &nbt::Value, path: &'static str) -> Option<f32> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Float(float) => Some(float),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_double(value: &nbt::Value, path: &'static str) -> Option<f64> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::Double(double) => Some(double),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_byte_array(value: &nbt::Value, path: &'static str) -> Option<Vec<i8>> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::ByteArray(byte_array) => Some(byte_array),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_string(value: &nbt::Value, path: &'static str) -> Option<String> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::String(string) => Some(string),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_list(value: &nbt::Value, path: &'static str) -> Option<Vec<nbt::Value>> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::List(list) => Some(list),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_int_array(value: &nbt::Value, path: &'static str) -> Option<Vec<i32>> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::IntArray(int_array) => Some(int_array),
        _ => None,
    })
}

#[allow(dead_code)]
pub fn nbt_value_lookup_long_array(value: &nbt::Value, path: &'static str) -> Option<Vec<i64>> {
    nbt_value_lookup(value, path).and_then(|value| match value {
        nbt::Value::LongArray(long_array) => Some(long_array),
        _ => None,
    })
}
