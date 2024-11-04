use crate::term::*;
use crate::util::text::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Error, ()> for Integer {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        if let Ok((text, n)) = isize::eat(text, ()) {
            return Ok((text, i(n)));
        }
        if let Ok(text) = '+'.drop(text) {
            return Ok((text, add()));
        }
        if let Ok(text) = '*'.drop(text) {
            return Ok((text, mul()));
        }
        Err(Error::Invalid)
    }
}
