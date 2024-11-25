use eat::text::*;

#[derive(Debug)]
pub enum NameCharError {
    Missing,
    Invalid,
}

pub struct NameChar(pub char);

impl Eat<NameCharError, ()> for NameChar {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), NameCharError> {
        use NameCharError::*;
        let (text, c) = char::eat(text, ()).map_err(|_| Missing)?;
        if !(c.is_alphanumeric() || c == '_') {
            return Err(Invalid);
        }
        Ok((text, NameChar(c)))
    }
}

#[derive(Debug)]
pub enum Error {
    FirstMissing,
    FirstNotAlphabetic,
}

pub struct Term(pub String);

impl Eat<Error, ()> for Term {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        use Error::*;
        let (text, c) = char::eat(text, ()).map_err(|_| FirstMissing)?;
        if !c.is_alphabetic() {
            return Err(FirstNotAlphabetic);
        }
        let (text, rest) = NameChar::eat_many(text, ());
        let mut name = String::new();
        name.push(c);
        name.extend(rest.into_iter().map(|x| x.0));
        Ok((text, Self(name)))
    }
}
