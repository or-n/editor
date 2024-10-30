use crate::term::*;
use crate::term_text::token::*;
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, ()> for BTerm {
    fn eat(i: &[Token], data: ()) -> Result<(&[Token], Self), Error> {
        Err(Error::Invalid)
    }
}
