use crate::term::{pair, r#abstract, r#let, tag, T};
use crate::util::eat_id;
use eat::*;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct EatData {
    pub parens: bool,
}

pub struct ThenNotLet(T);
pub struct Then(T);

impl Eat<&str, (), Arc<str>> for ThenNotLet {
    fn eat(i: &str, sep: Arc<str>) -> Result<(&str, Self), ()> {
        eat_then_not_let(i, sep)
    }
}

impl Eat<&str, (), Arc<str>> for Then {
    fn eat(i: &str, sep: Arc<str>) -> Result<(&str, Self), ()> {
        eat_then(i, sep)
    }
}

fn eat_then_not_let(i: &str, sep: Arc<str>) -> Result<(&str, ThenNotLet), ()> {
    let data = EatData { parens: false };
    let i = sep.as_ref().drop(i)?;
    let (i, t) = eat_not_let(i, data)?;
    Ok((i, ThenNotLet(t)))
}

fn eat_then(i: &str, sep: Arc<str>) -> Result<(&str, Then), ()> {
    let data = EatData { parens: false };
    let i = sep.as_ref().drop(i)?;
    let (i, t) = eat(i, data)?;
    Ok((i, Then(t)))
}

pub fn eat(i: &str, data: EatData) -> Result<(&str, T), ()> {
    if !data.parens {
        let data2 = EatData { parens: true };
        if let Ok((i, start)) = eat_not_let(i, data2) {
            let (i, rest) = ThenNotLet::eat_many(i, " ".into());
            if !rest.is_empty() {
                let rest_iter = rest.into_iter().map(|ThenNotLet(x)| x);
                let x = rest_iter.fold(start, |out, x| r#let(out, x));
                return Ok((i, x));
            }
        }
    }
    eat_not_let(i, data)
}

pub fn eat_not_let(i: &str, data: EatData) -> Result<(&str, T), ()> {
    use T::*;
    if let Ok(i) = '('.drop(i) {
        let data = EatData { parens: false };
        if let Ok((i, t)) = eat(i, data) {
            if let Ok(i) = ", ".drop(i) {
                let (i, t2) = eat(i, data)?;
                let i = ')'.drop(i)?;
                return Ok((i, pair(t, t2)));
            }
            let i = ')'.drop(i)?;
            return Ok((i, t));
        }
    }
    if let Ok(i) = '['.drop(i) {
        let data = EatData { parens: false };
        if let Ok((i, start)) = eat(i, data) {
            let (i, rest) = Then::eat_many(i, ", ".into());
            let rest_iter = rest.into_iter().map(|Then(x)| x);
            let r = std::iter::once(start).chain(rest_iter).collect();
            let i = ']'.drop(i)?;
            return Ok((i, Array(r)));
        }
        let i = ']'.drop(i)?;
        return Ok((i, Array(vec![])));
    }
    if !data.parens {
        if let Ok(i) = "let ".drop(i) {
            let data2 = EatData { parens: true };
            if let Ok((i, t1)) = eat(i, data2) {
                let i = ' '.drop(i)?;
                let (i, t2) = eat(i, data)?;
                return Ok((i, r#let(t1, t2)));
            }
        }
    }
    if let Ok((i, x)) = u32::eat(i, ()) {
        return Ok((i, I(x as i64)));
    }
    if let Ok((i, id)) = eat_id(i) {
        if !data.parens {
            if let Ok(i) = ": ".drop(i) {
                let data = EatData { parens: false };
                let (i, t) = eat(i, data)?;
                return Ok((i, r#abstract(id, t)));
            }
        }
        match id.as_str() {
            "let" => {}
            "no" => return Ok((i, tag(0))),
            "ok" => return Ok((i, tag(1))),
            _ => return Ok((i, Id(id))),
        }
    }
    Err(())
}
