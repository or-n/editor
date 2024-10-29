pub mod integer;
pub mod pair;

use crate::term::*;
use crate::text::*;

#[derive(Debug, Clone, Copy)]
pub struct Settings {}

#[derive(Debug)]
pub enum TermError {
    Invalid,
}

impl<'a> Eat<'a, TermError, Settings> for BTerm {
    fn eat(text: &'a str, data: Settings) -> Result<(&'a str, Self), TermError> {
        if let Ok((text, term)) = pair::Term::eat(text, data) {
            return Ok((text, term.0));
        }
        if let Ok((text, term)) = Integer::eat(text, ()) {
            return Ok((text, integer(term)));
        }
        Err(TermError::Invalid)
    }
}
