use crate::term::*;

pub fn id(x: String) -> T {
    T::Id(x)
}

pub fn r#abstract(a: String, b: T) -> T {
    T::Abstract(a, Box::new(b))
}

pub fn r#let(a: T, b: T) -> T {
    T::Let(Box::new(a), Box::new(b))
}

pub fn i(x: i64) -> T {
    T::I(x)
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
