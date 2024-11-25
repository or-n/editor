use crate::term::*;
use eat::*;

use crate::term_from_text::{settings::*, token::Token};

#[derive(Debug)]
pub enum Error {
    Keyword,
    Space,
    A(super::Error),
    Newline,
}

pub struct Term(pub BTerm);

impl Eat<&[Token], Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        use Token::*;
        let i = Name("if".to_string()).drop(i).map_err(|_| Keyword)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Whitespace('\n', 1).drop(i).map_err(|_| Newline)?;
        let (i, b) = Branch::eat_many(i, ());
        Ok((i, Self(r#if(a, b))))
    }
}

#[derive(Debug)]
pub enum BranchError {
    Indent,
    Tag,
    Space1,
    LetName,
    Colon,
    Space2,
    Block(super::Error),
    Newline,
}

impl Eat<&[Token], BranchError, ()> for Branch {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), BranchError> {
        use BranchError::*;
        use Token::*;
        let i = Whitespace('\t', 1).drop(i).map_err(|_| Indent)?;
        let (i, tag) = match Token::eat(i, ()) {
            Ok((i, Integer(n))) => (i, n),
            _ => return Err(Tag),
        };
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space1)?;
        let (i, name) = match Token::eat(i, ()) {
            Ok((i, Name(n))) => (i, n),
            _ => return Err(LetName),
        };
        let i = Special(':').drop(i).map_err(|_| Colon)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space2)?;
        let (i, block) = BTerm::eat(i, Settings::all(true)).map_err(Block)?;
        let i = Whitespace('\n', 1).drop(i).map_err(|_| Newline)?;
        Ok((i, Branch { tag, name, block }))
    }
}
