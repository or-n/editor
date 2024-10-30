pub mod integer;

use crate::term::*;
use crate::term_text::token::*;
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, ()> for BTerm {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        if let Ok((i, term)) = Integer::eat(i, ()) {
            return Ok((i, integer(term)));
        }
        Err(Error::Invalid)
    }
}
