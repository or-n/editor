
pub fn set_vec<T: serde::Serialize>(values: &Vec<T>) -> Vec<u8> {
    values.iter().flat_map(|value| bincode::serialize(value).unwrap()).collect()
}

pub fn get_vec<T: serde::de::DeserializeOwned>(bytes: &Vec<u8>)
-> Result<Vec<T>, Box<bincode::ErrorKind>> {
    let mut values = vec![];
    let mut cursor = &bytes[..];
    while !cursor.is_empty() {
        values.push(bincode::deserialize_from(&mut cursor)?)
    }
    Ok(values)
}
