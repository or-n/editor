pub mod integer;

use crate::term::*;
use crate::text::*;

#[derive(Debug)]
pub struct Settings {}

#[derive(Debug)]
pub enum TermError {
    Invalid,
}

impl<'a> Eat<'a, TermError, Settings> for BTerm {
    fn eat(text: &'a str, _data: Settings) -> Result<(&'a str, Self), TermError> {
        if let Ok((text, i)) = Integer::eat(text, ()) {
            return Ok((text, integer(i)));
        }
        Err(TermError::Invalid)
    }
}
