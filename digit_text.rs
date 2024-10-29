use crate::text::*;

pub struct Digit(u32);

impl<'a> TryFrom<Value<&'a str>> for Value<Digit> {
    type Error = ();

    fn try_from(input: Value<&'a str>) -> Result<Self, Self::Error> {
        let c = input.index.pop_char(input.value).ok_or(())?;
        let digit = c.value.to_digit(10).ok_or(())?;
        Ok(c.map(|_| Digit(digit)))
    }
}

impl<'a> TryFrom<Value<&'a str>> for Value<isize> {
    type Error = ();

    fn try_from(input: Value<&'a str>) -> Result<Self, Self::Error> {
        let digits = input.index.many::<Digit>(input.value);
        if digits.value.is_empty() {
            return Err(());
        }
        Ok(digits.map(|value| {
            value
                .into_iter()
                .fold(0, |r, digit| r * 10 + digit.0 as isize)
        }))
    }
}

impl<'a> Eat<'a, (), ()> for Digit {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), ()> {
        let (i, c) = char::eat(text, i, ())?;
        let digit = c.to_digit(10).ok_or(())?;
        Ok((i, Digit(digit)))
    }
}

impl<'a> Eat<'a, (), ()> for isize {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), ()> {
        let (i, digits) = Vec::<Digit>::eat(text, i, ())?;
        if digits.is_empty() {
            return Err(());
        }
        let n = digits
            .into_iter()
            .fold(0, |r, digit| r * 10 + digit.0 as isize);
        Ok((i, n))
    }
}
