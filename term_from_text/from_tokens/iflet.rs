use crate::term::*;
use eat::token::*;

use crate::term_from_text::{settings::*, token::Token};

#[derive(Debug)]
pub enum Error {
    Tag,
    Space1,
    LetName,
    Colon,
    Space2,
    Keyword,
    Space3,
    A(super::Error),
    Newline,
    Block(super::Error),
}

pub struct Term(pub BTerm);

impl Eat<Token, Error, ()> for Term {
    fn eat(i: &[Token], _data: ()) -> Result<(&[Token], Self), Error> {
        use Error::*;
        use Token::*;
        let (i, tag) = super::eat_isize(i).ok_or(Tag)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space1)?;
        let (i, name) = super::eat_name(i).ok_or(LetName)?;
        let i = Special(':').drop(i).map_err(|_| Colon)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space2)?;
        let i = Name("if".to_string()).drop(i).map_err(|_| Keyword)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space3)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Whitespace('\n', 1).drop(i).map_err(|_| Newline)?;
        let (i, b) = Branch::eat_many(i, ());
        let (i, block) = BTerm::eat(i, Settings::all(true)).map_err(Block)?;
        Ok((i, Self(iflet(a, b, Branch { tag, name, block }))))
    }
}
