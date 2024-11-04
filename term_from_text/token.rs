use crate::term_from_text::name;
use crate::util::text::*;
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

impl Eat<Error, ()> for Token {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), Error> {
        if let Ok((text, token)) = Special::eat(text, ()) {
            return Ok((text, Token::Special(token.0)));
        }
        if let Ok((text, token)) = Whitespace::eat(text, ()) {
            return Ok((text, Token::Whitespace(token.0, token.1)));
        }
        if let Ok((text, token)) = name::Term::eat(text, ()) {
            return Ok((text, Token::Name(token.0)));
        }
        if let Ok((text, token)) = isize::eat(text, ()) {
            return Ok((text, Token::Integer(token)));
        }
        Err(Error::Invalid)
    }
}

struct Special(char);

impl Eat<(), ()> for Special {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (text, c) = char::eat(text, ())?;
        if !".,:()+*".contains(c) {
            return Err(());
        }
        Ok((text, Special(c)))
    }
}

struct Whitespace(char, usize);

impl Eat<(), ()> for Whitespace {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (text, c) = char::eat(text, ())?;
        if !c.is_whitespace() {
            return Err(());
        }
        let (text, count) = c.drop_many(text);
        Ok((text, Whitespace(c, count + 1)))
    }
}
