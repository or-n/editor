use crate::term::*;
use eat::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<&str, Error, ()> for Integer {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), Error> {
        if let Ok((i, n)) = u32::eat(i, ()) {
            return Ok((i, crate::term::i(n as isize)));
        }
        if let Ok(i) = '+'.drop(i) {
            return Ok((i, add()));
        }
        if let Ok(i) = '*'.drop(i) {
            return Ok((i, mul()));
        }
        Err(Error::Invalid)
    }
}
