#[allow(dead_code)]
#[derive(Debug)]
pub enum CharError {
    Missing,
    Mismatch { expected: char, got: char },
}

pub trait Eat<Error, Data>
where
    Self: Sized,
{
    fn eat(text: &str, data: Data) -> Result<(&str, Self), Error>;
}

pub trait EatMany<Error, Data>
where
    Self: Sized,
{
    fn eat_many(text: &str, data: Data) -> (&str, Vec<Self>);
}

impl<Error, Data, T> EatMany<Error, Data> for T
where
    T: Eat<Error, Data>,
    Data: Copy,
{
    fn eat_many(mut text: &str, data: Data) -> (&str, Vec<T>) {
        let mut results = vec![];
        while let Ok((new_text, item)) = T::eat(text, data) {
            text = new_text;
            results.push(item)
        }
        (text, results)
    }
}

pub trait Drop<Error>
where
    Self: Sized,
{
    fn drop(self, text: &str) -> Result<&str, Error>;
}

impl Eat<(), ()> for char {
    fn eat(s: &str, _data: ()) -> Result<(&str, char), ()> {
        let c = s.chars().next().ok_or(())?;
        Ok((&s[c.len_utf8()..], c))
    }
}

impl Drop<()> for char {
    fn drop(self, s: &str) -> Result<&str, ()> {
        let c = s.chars().next().ok_or(())?;
        if c != self {
            return Err(());
        }
        Ok(&s[c.len_utf8()..])
    }
}

impl<'b> Drop<()> for &'b str {
    fn drop(self, mut s: &str) -> Result<&str, ()> {
        for c in self.chars() {
            s = c.drop(s)?;
        }
        Ok(s)
    }
}
