#[macro_export]
macro_rules! update { ($x:ty) => (impl Fn($x) -> $x) }

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
