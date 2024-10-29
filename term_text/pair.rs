use super::settings::*;
use super::*;

pub struct Term(pub BTerm);

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    L(CharError),
    A(TermError),
    Sep,
    B(TermError),
    R(CharError),
}

impl<'a> TryFrom<Value<(&'a str, Settings)>> for Value<Term> {
    type Error = Error;

    fn try_from(input: Value<(&'a str, Settings)>) -> Result<Self, Self::Error> {
        use Error::*;
        let text = input.value.0;
        let l = Value::<char>::try_from(input.index.value((text, '('))).map_err(L)?;
        let a = Value::<BTerm>::try_from(l.index.value(input.value.clone())).map_err(A)?;
        let sep = a.index.tag(", ", text).ok_or(Sep).map_err(|_| Sep)?;
        let b = Value::<BTerm>::try_from(sep.index.value(input.value.clone())).map_err(B)?;
        let r = Value::<char>::try_from(b.index.value((text, ')'))).map_err(R)?;
        Ok(r.index.value(Term(pair(a.value, b.value))))
    }
}
