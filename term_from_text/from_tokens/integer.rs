use crate::term::*;
use crate::term_from_text::token::*;
use eat::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, ()> for Integer {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        let (i, token) = Token::eat(i, ()).map_err(|_| Error::Invalid)?;
        use Token::*;
        match token {
            Integer(n) => Ok((i, crate::term::i(n))),
            Special('+') => Ok((i, add())),
            Special('*') => Ok((i, mul())),
            _ => Err(Error::Invalid),
        }
    }
}
