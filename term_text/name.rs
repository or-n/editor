use crate::text::*;

struct NameChar(char);

impl<'a> TryFrom<Value<&'a str>> for Value<NameChar> {
    type Error = ();

    fn try_from(input: Value<&'a str>) -> Result<Self, Self::Error> {
        let c = input.index.pop_char(input.value).ok_or(())?;
        if !(c.value.is_alphanumeric() || c.value == '_') {
            return Err(());
        }
        Ok(c.map(NameChar))
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum NameError {
    FirstMissing,
    FirstNotAlphabetic(char),
}

impl<'a> TryFrom<Value<&'a str>> for Value<String> {
    type Error = NameError;

    fn try_from(input: Value<&'a str>) -> Result<Self, Self::Error> {
        use NameError::*;
        let c = input.index.pop_char(input.value).ok_or(FirstMissing)?;
        if !c.value.is_alphabetic() {
            return Err(FirstNotAlphabetic(c.value));
        }
        let rest = c.index.many::<NameChar>(input.value);
        let mut name = String::new();
        name.push(c.value);
        name.extend(rest.value.into_iter().map(|x| x.0));
        Ok(rest.index.value(name))
    }
}

impl<'a> Eat<'a, (), ()> for NameChar {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), ()> {
        let (i, c) = char::eat(text, i, ())?;
        if !(c.is_alphanumeric() || c == '_') {
            return Err(());
        }
        Ok((i, NameChar(c)))
    }
}

impl<'a> Eat<'a, (), NameError> for String {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), NameError> {
        use NameError::*;
        let (i, first) = char::eat(text, i, ()).map_err(|_| FirstMissing)?;
        if !first.is_alphanumeric() {
            return Err(FirstNotAlphabetic(first));
        }
        let (i, rest) = eat_many::<NameChar, _, _>(text, i, ());
        let mut name = String::new();
        name.push(first);
        name.extend(rest.into_iter().map(|x| x.0));
        Ok((i, name))
    }
}
