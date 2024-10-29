use super::*;

pub struct Term(pub BTerm);

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApplyError {
    A(TermError),
    Sep(CharError),
    B(TermError),
}

impl<'a> TryFrom<Value<(&'a str, Settings)>> for Value<Term> {
    type Error = ApplyError;

    fn try_from(input: Value<(&'a str, Settings)>) -> Result<Self, Self::Error> {
        use ApplyError::*;
        let text = input.value.0;
        let mut settings = input.value.1;
        settings.apply = false;
        let a = Value::<BTerm>::try_from(input.index.value((text, settings))).map_err(A)?;
        let c = Value::<char>::try_from(a.index.value((text, ' '))).map_err(Sep)?;
        let b = Value::<BTerm>::try_from(c.index.value(input.value.clone())).map_err(B)?;
        Ok(b.index.value(Term(apply(a.value, b.value))))
    }
}
