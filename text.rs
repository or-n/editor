#[allow(dead_code)]
#[derive(Debug)]
pub enum CharError {
    Missing,
    Mismatch { expected: char, got: char },
}

pub trait Eat<'a, Error, Data>
where
    Self: Sized,
{
    fn eat(text: &'a str, data: Data) -> Result<(&'a str, Self), Error>;
}

pub trait EatMany<'a, Error, Data>
where
    Self: Sized,
{
    fn eat_many(text: &'a str, data: Data) -> (&'a str, Vec<Self>);
}

impl<'a, Error, Data, T> EatMany<'a, Error, Data> for T
where
    T: Eat<'a, Error, Data>,
    Data: Copy,
{
    fn eat_many(mut text: &'a str, data: Data) -> (&'a str, Vec<T>) {
        let mut results = vec![];
        while let Ok((new_text, item)) = T::eat(text, data) {
            text = new_text;
            results.push(item)
        }
        (text, results)
    }
}

pub trait Drop<'a, Error>
where
    Self: Sized,
{
    fn drop(self, text: &'a str) -> Result<&'a str, Error>;
}

impl<'a> Eat<'a, (), ()> for char {
    fn eat(s: &str, _data: ()) -> Result<(&str, char), ()> {
        let c = s.chars().next().ok_or(())?;
        Ok((&s[c.len_utf8()..], c))
    }
}

impl<'a> Drop<'a, ()> for char {
    fn drop(self, s: &str) -> Result<&str, ()> {
        let c = s.chars().next().ok_or(())?;
        if c != self {
            return Err(());
        }
        Ok(&s[c.len_utf8()..])
    }
}

impl<'a, 'b> Drop<'a, ()> for &'b str {
    fn drop(self, mut s: &'a str) -> Result<&'a str, ()> {
        for c in self.chars() {
            s = c.drop(s)?;
        }
        Ok(s)
    }
}
