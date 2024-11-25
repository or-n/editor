pub mod apply;
pub mod from_tokens;
pub mod integer;
pub mod r#let;
pub mod name;
pub mod pair;
pub mod settings;
pub mod token;

use crate::term::*;
use eat::*;
use settings::*;

#[derive(Debug)]
pub enum TermError {
    Invalid,
}

impl Eat<&str, TermError, Settings> for BTerm {
    fn eat(i: &str, data: Settings) -> Result<(&str, Self), TermError> {
        if let Ok((i, term)) = r#let::Term::eat(i, ()) {
            return Ok((i, term.0));
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
        if let Ok((i, term)) = name::Term::eat(i, ()) {
            return Ok((i, parameter(term.0)));
        }
        Err(TermError::Invalid)
    }
}
