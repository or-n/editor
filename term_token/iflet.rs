use crate::term::*;
use crate::token::*;

use crate::term_text::{settings::*, token::Token};

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
        let i = Name("if".to_string()).drop(i).map_err(|_| Keyword)?;
        let i = Whitespace(' ', 1).drop(i).map_err(|_| Space3)?;
        let (i, a) = BTerm::eat(i, Settings::all(true)).map_err(A)?;
        let i = Whitespace('\n', 1).drop(i).map_err(|_| Newline)?;
        let (i, b) = <(isize, Branch)>::eat_many(i, ());
        let (i, block) = BTerm::eat(i, Settings::all(true)).map_err(Block)?;
        Ok((i, Self(iflet(a, b, (tag, Branch { name, block })))))
    }
}
