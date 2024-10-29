use crate::term::*;
use crate::text::*;

impl<'a> TryFrom<Value<&'a str>> for Value<Integer> {
    type Error = ();

    fn try_from(input: Value<&'a str>) -> Result<Self, Self::Error> {
        if let Ok(t) = Value::<isize>::try_from(input.clone()) {
            return Ok(t.map(i));
        }
        if let Some(t) = input.index.tag("+", input.value) {
            return Ok(t.map(|_| add()));
        }
        if let Some(t) = input.index.tag("*", input.value) {
            return Ok(t.map(|_| mul()));
        }
        Err(())
    }
}

impl<'a> Eat<'a, (), ()> for Integer {
    fn eat(text: &'a str, i: Index, _data: ()) -> Result<(Index, Self), ()> {
        if let Ok((i, n)) = isize::eat(text, i, ()) {
            return Ok((i, Integer::I(n)));
        }
        if let Ok((i, _)) = char::eat(text, i, '+') {
            return Ok((i, add()));
        }
        if let Ok((i, _)) = char::eat(text, i, '*') {
            return Ok((i, mul()));
        }
        Err(())
    }
}

struct Or<T>(T);

impl<'a, Data, T, Error> Eat<'a, Vec<(Data, T)>, Option<Error>> for Or<T>
where
    T: Eat<'a, Data, Error>,
{
    fn eat(text: &'a str, i: Index, data: Vec<(Data, T)>) -> Result<(Index, Self), Option<Error>> {
        for (d, r) in data.into_iter() {
            if let Ok((i, _)) = T::eat(text, i, d) {
                return Ok((i, Or(r)));
            }
        }
        return Err(None);
    }
}
