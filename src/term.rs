#[derive(Debug, Clone)]
pub enum T {
    Id(String),
    Abstract(String, Box<T>),
    Let(Box<T>, Box<T>),
    I(i64),
    Pair(Box<T>, Box<T>),
    Array(Vec<T>), // cannot be if, since only output is same, not input
}

pub fn r#abstract(a: String, b: T) -> T {
    T::Abstract(a, Box::new(b))
}

pub fn r#let(a: T, b: T) -> T {
    T::Let(Box::new(a), Box::new(b))
}

pub fn pair(a: T, b: T) -> T {
    T::Pair(Box::new(a), Box::new(b))
}

pub fn tag(i: i64) -> T {
    use T::*;
    let x = "x".to_string();
    let tag = pair(I(i), Id(x.clone()));
    r#abstract(x, tag)
}
