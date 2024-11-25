pub mod apply;
pub mod r#if;
pub mod iflet;
pub mod infixl;
pub mod infixr;
pub mod integer;
pub mod r#let;
pub mod pair;

use crate::term::*;
use crate::term_from_text::{settings::*, token::Token};
use eat::token::*;

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<Token, Error, Settings> for BTerm {
    fn eat(i: &[Token], data: Settings) -> Result<(&[Token], Self), Error> {
        if let Ok((i, term)) = r#if::Term::eat(i, ()) {
            return Ok((i, term.0));
        }
        if let Ok((i, term)) = iflet::Term::eat(i, ()) {
            return Ok((i, term.0));
        }
        if let Ok((i, term)) = r#let::Term::eat(i, ()) {
            return Ok((i, term.0));
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
        if data.apply {
            if let Ok((i, term)) = apply::Term::eat(i, ()) {
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

pub fn sep(i: &[Token]) -> Result<&[Token], ()> {
    if let Ok(i) = Token::Whitespace(' ', 1).drop(i) {
        return Ok(i);
    }
    let i = Token::Whitespace('\n', 1).drop(i).map_err(|_| ())?;
    let i = Token::Whitespace('\t', 1).drop(i).map_err(|_| ())?;
    Ok(i)
}

pub fn eat_name(i: &[Token]) -> Option<(&[Token], String)> {
    let (i, token) = Token::eat(i, ()).ok()?;
    Some((i, token.as_name()?.to_string()))
}

pub fn eat_isize(i: &[Token]) -> Option<(&[Token], isize)> {
    let (i, token) = Token::eat(i, ()).ok()?;
    Some((i, token.as_integer()?.clone()))
}
