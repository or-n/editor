use std::fs::{read, write, remove_file};
use std::io::Result as IO;

use crate::update;

pub fn optional_file(path: &std::path::Path, f: update!{Option<Vec<u8>>}) -> IO<()> {
    match f(if path.exists() { Some(read(path)?) } else { None }) {
        Some(new_contents) => write(path, new_contents),
        None => remove_file(path)
    }
}

