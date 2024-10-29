use crate::term::*;
use crate::text::*;

pub mod apply;
pub mod integer;
pub mod name;
pub mod pair;
pub mod settings;

use settings::*;

#[derive(Debug)]
pub enum TermError {
    End,
    Invalid,
}

impl<'a> TryFrom<Value<(&'a str, Settings)>> for Value<BTerm> {
    type Error = TermError;

    fn try_from(input: Value<(&'a str, Settings)>) -> Result<Self, Self::Error> {
        use TermError::*;
        let text = input.value.0;
        let settings = input.value.1;
        if input.index.value >= text.len() {
            return Err(End);
        }
        if let Ok(t) = Value::<pair::Term>::try_from(input.clone()) {
            return Ok(t.map(|x| x.0));
        }
        if settings.apply {
            if let Ok(t) = Value::<apply::Term>::try_from(input.clone()) {
                return Ok(t.map(|x| x.0));
            }
        }
        if let Ok((i, t)) = String::eat(text, input.index, ()) {
            return Ok(i.value(parameter(t)));
        }
        if let Ok((i, t)) = Integer::eat(text, input.index, ()) {
            return Ok(i.value(integer(t)));
        }
        Err(Invalid)
    }
}
