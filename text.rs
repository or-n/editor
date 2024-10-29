use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub value: usize,
}

impl Index {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn peek_char(&self, input: &str) -> Option<char> {
        input[self.value..].chars().next()
    }

    pub fn pop_char(self, input: &str) -> Option<Value<char>> {
        let c = self.peek_char(input)?;
        let new = Self {
            value: self.value + c.len_utf8(),
        };
        Some(new.value(c))
    }

    pub fn tag<'b>(mut self, x: &'b str, input: &str) -> Option<Value<&'b str>> {
        for valid_c in x.chars() {
            let c = self.pop_char(input)?;
            self = c.index;
            if valid_c != c.value {
                return None;
            }
        }
        Some(self.value(x))
    }

    pub fn many<'a, T>(mut self, input: &'a str) -> Value<Vec<T>>
    where
        Value<T>: TryFrom<Value<&'a str>>,
    {
        let mut results = vec![];
        while let Ok(t) = Value::<T>::try_from(self.value(input)) {
            self = t.index;
            results.push(t.value);
        }
        self.value(results)
    }

    pub fn value<T>(self, t: T) -> Value<T> {
        Value {
            index: self,
            value: t,
        }
    }

    pub fn get<T, E, Data>(self, data: Data) -> Result<Value<T>, E>
    where
        Value<T>: TryFrom<Value<Data>, Error = E>,
    {
        Value::<T>::try_from(self.value(data))
    }
}

#[derive(Clone)]
pub struct Value<T> {
    pub index: Index,
    pub value: T,
}

impl<T> Value<T> {
    pub fn map<NewT, F>(self, f: F) -> Value<NewT>
    where
        F: Fn(T) -> NewT,
    {
        Value {
            index: self.index,
            value: f(self.value),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CharError {
    Missing,
    Mismatch { expected: char, got: char },
}

impl<'a> TryFrom<Value<(&'a str, char)>> for Value<char> {
    type Error = CharError;

    fn try_from(input: Value<(&'a str, char)>) -> Result<Self, Self::Error> {
        let (text, expected) = input.value;
        let c = input.index.pop_char(text).ok_or(CharError::Missing)?;
        if c.value != expected {
            return Err(CharError::Mismatch {
                expected,
                got: c.value,
            });
        }
        Ok(c)
    }
}

impl<'a> Eat<'a, char, CharError> for char {
    fn eat(text: &'a str, i: Index, data: char) -> Result<(Index, Self), CharError> {
        let (i, c) = char::eat(text, i, ()).map_err(|_| CharError::Missing)?;
        if c != data {
            return Err(CharError::Mismatch {
                expected: data,
                got: c,
            });
        }
        Ok((i, c))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum StrError {
    SoFar(Index, CharError),
}

impl<'a, 'b> Eat<'a, &'b str, StrError> for &'b str {
    fn eat(text: &'a str, mut i: Index, data: &'b str) -> Result<(Index, Self), StrError> {
        for c in data.chars() {
            let (new_i, _) = char::eat(text, i, c).map_err(|e| StrError::SoFar(i, e))?;
            i = new_i;
        }
        Ok((i, data))
    }
}

pub trait Eat<'a, Data, Error>
where
    Self: Sized,
{
    fn eat(text: &'a str, i: Index, data: Data) -> Result<(Index, Self), Error>;
}

impl<'a> Eat<'a, (), ()> for char {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), ()> {
        let c = text[i.value..].chars().next().ok_or(())?;
        let i = Index {
            value: i.value + c.len_utf8(),
        };
        Ok((i, c))
    }
}

impl<'a, T, Data, Error> Eat<'a, Data, Error> for Vec<T>
where
    T: Eat<'a, Data, Error>,
    Data: Copy,
{
    fn eat(text: &'a str, mut i: Index, data: Data) -> Result<(Index, Self), Error> {
        let mut results = vec![];
        while let Ok((new_i, value)) = T::eat(text, i, data) {
            i = new_i;
            results.push(value);
        }
        Ok((i, results))
    }
}

pub fn eat_many<'a, T, Data, Error>(text: &'a str, mut i: Index, data: Data) -> (Index, Vec<T>)
where
    T: Eat<'a, Data, Error>,
    Data: Copy,
{
    let mut results = vec![];
    while let Ok((new_i, value)) = T::eat(text, i, data) {
        i = new_i;
        results.push(value);
    }
    (i, results)
}

pub trait Eat2<'a, Error, Data>
where
    Self: Sized,
{
    fn eat2(text: &'a str, data: Data) -> Result<(&'a str, Self), Error>;
}

pub trait Drop<'a, Error>
where
    Self: Sized,
{
    fn drop(self, text: &'a str) -> Result<&'a str, Error>;
}

impl<'a> Eat2<'a, (), ()> for char {
    fn eat2(s: &str, _data: ()) -> Result<(&str, char), ()> {
        let c = s.chars().next().ok_or(())?;
        Ok((&s[c.len_utf8()..], c))
    }
}

impl<'a> Drop<'a, ()> for char {
    fn drop(self, s: &str) -> Result<&str, ()> {
        let (s, c) = char::eat2(s, ())?;
        if c != self {
            return Err(());
        }
        Ok(s)
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
