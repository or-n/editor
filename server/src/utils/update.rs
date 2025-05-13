use std::fs::{read, write, remove_file};
use std::io::Result as IO;

#[macro_export]
macro_rules! update { ($x:ty) => (impl Fn($x) -> $x) }

pub fn optional_file(path: &std::path::Path, f: update!{Option<Vec<u8>>}) -> IO<()> {
    match f(if path.exists() { Some(read(path)?) } else { None }) {
        Some(new_contents) => write(path, new_contents),
        None => remove_file(path)
    }
}

pub fn string_empty_option(f: update!{String}) -> update!{Option<Vec<u8>>} {
    move |x| {
        let bytes = x.unwrap_or(vec![]);
        let result = f(String::from_utf8(bytes).unwrap());
        if result.is_empty() {
            None
        } else {
            Some(result.into_bytes())
        }
    }
}
