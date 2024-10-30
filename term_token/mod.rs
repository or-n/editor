pub mod apply;
pub mod infixl;
pub mod infixr;
pub mod integer;
pub mod r#let;
pub mod name;
pub mod pair;

use crate::term::*;
use crate::term_text::{settings::*, token::*};
use crate::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, Settings> for BTerm {
    fn eat(i: &[Token], data: Settings) -> Result<(&[Token], Self), Error> {
        if let Ok((i, term)) = r#let::Term::eat(i, ()) {
            return Ok((i, term.0));
        }
        if data.apply {
            if let Ok((i, term)) = apply::Term::eat(i, ()) {
                return Ok((i, term.0));
            }
        }
        if data.infixl {
            if let Ok((i, term)) = infixl::Term::eat(i, ()) {
                return Ok((i, term.0));
            }
        }
        if data.infixr {
            if let Ok((i, term)) = infixr::Term::eat(i, ()) {
                return Ok((i, term.0));
            }
        }
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
