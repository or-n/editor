#[derive(Debug, Clone)]
pub enum T {
    Id(String),
    Abstract(String, Box<T>),
    Let(Box<T>, Box<T>),
    I(i64),
    Pair(Box<T>, Box<T>),
    If(Vec<T>),
}
