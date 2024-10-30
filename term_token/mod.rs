pub mod integer;
pub mod pair;

use crate::term::*;
use crate::term_text::token::*;
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, ()> for BTerm {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        if let Ok((i, term)) = pair::Term::eat(i, ()) {
            return Ok((i, term.0));
        }
        if let Ok((i, term)) = Integer::eat(i, ()) {
            return Ok((i, integer(term)));
        }
        if let Ok((i, Token::Name(term))) = Token::eat(i, ()) {
            return Ok((i, parameter(term)));
        }
        Err(Error::Invalid)
    }
}
