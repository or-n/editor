pub mod apply;
pub mod integer;
pub mod r#let;
pub mod name;
pub mod pair;
pub mod settings;
pub mod token;

use crate::term::*;
use crate::text::*;
use settings::*;

#[derive(Debug)]
pub enum TermError {
    Invalid,
}

impl Eat<TermError, Settings> for BTerm {
    fn eat(text: &str, data: Settings) -> Result<(&str, Self), TermError> {
        if let Ok((text, term)) = r#let::Term::eat(text, ()) {
            return Ok((text, term.0));
        }
        if data.apply {
            if let Ok((text, term)) = apply::Term::eat(
                text,
                Settings {
                    apply: false,
                    ..data
                },
            ) {
                return Ok((text, term.0));
            }
        }
        if let Ok((text, term)) = pair::Term::eat(text, ()) {
            return Ok((text, term.0));
        }
        if let Ok((text, term)) = Integer::eat(text, ()) {
            return Ok((text, integer(term)));
        }
        if let Ok((text, term)) = name::Term::eat(text, ()) {
            return Ok((text, parameter(term.0)));
        }
        Err(TermError::Invalid)
    }
}
