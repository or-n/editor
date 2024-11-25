use eat::*;

#[derive(Debug)]
pub enum NameCharError {
    Missing,
    Invalid,
}

pub struct NameChar(pub char);

impl Eat<&str, NameCharError, ()> for NameChar {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), NameCharError> {
        use NameCharError::*;
        let (i, c) = char::eat(i, ()).map_err(|_| Missing)?;
        if !(c.is_alphanumeric() || c == '_') {
            return Err(Invalid);
        }
        Ok((i, NameChar(c)))
    }
}

#[derive(Debug)]
pub enum Error {
    FirstMissing,
    FirstNotAlphabetic,
}

pub struct Term(pub String);

impl Eat<&str, Error, ()> for Term {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), Error> {
        use Error::*;
        let (i, c) = char::eat(i, ()).map_err(|_| FirstMissing)?;
        if !c.is_alphabetic() {
            return Err(FirstNotAlphabetic);
        }
        let (i, rest) = NameChar::eat_many(i, ());
        let mut name = String::new();
        name.push(c);
        name.extend(rest.into_iter().map(|x| x.0));
        Ok((i, Self(name)))
    }
}
