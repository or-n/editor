use crate::term_from_text::name;
use eat::*;
use enum_as_inner::EnumAsInner;

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum Token {
    Special(char),
    Whitespace(char, usize),
    Name(String),
    Integer(isize),
}

#[derive(Debug)]
pub enum Error {
    Invalid,
}

impl Eat<&str, Error, ()> for Token {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), Error> {
        if let Ok((i, token)) = Special::eat(i, ()) {
            return Ok((i, Token::Special(token.0)));
        }
        if let Ok((i, token)) = Whitespace::eat(i, ()) {
            return Ok((i, Token::Whitespace(token.0, token.1)));
        }
        if let Ok((i, token)) = name::Term::eat(i, ()) {
            return Ok((i, Token::Name(token.0)));
        }
        if let Ok((i, token)) = u32::eat(i, ()) {
            return Ok((i, Token::Integer(token as isize)));
        }
        Err(Error::Invalid)
    }
}

struct Special(char);

impl Eat<&str, (), ()> for Special {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (i, c) = char::eat(i, ())?;
        if !".,:()+*".contains(c) {
            return Err(());
        }
        Ok((i, Special(c)))
    }
}

struct Whitespace(char, usize);

impl Eat<&str, (), ()> for Whitespace {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (i, c) = char::eat(i, ())?;
        if !c.is_whitespace() {
            return Err(());
        }
        let (i, count) = c.drop_many(i);
        Ok((i, Whitespace(c, count + 1)))
    }
}
