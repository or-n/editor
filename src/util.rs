use eat::*;

pub fn eat_id(i: &str) -> Result<(&str, String), ()> {
    let (i, start) = eat_alpha(i)?;
    let (i, rest) = AlphaNum::eat_many(i, ());
    let rest_iter = rest.into_iter().map(|AlphaNum(x)| x);
    let id = std::iter::once(start).chain(rest_iter).collect();
    Ok((i, id))
}

fn eat_alpha(i: &str) -> Result<(&str, char), ()> {
    if let Ok((i, x)) = char::eat(i, ()) {
        if x.is_alphabetic() {
            return Ok((i, x));
        }
    }
    Err(())
}

pub struct AlphaNum(char);

impl Eat<&str, (), ()> for AlphaNum {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        eat_alphanum(i)
    }
}

fn eat_alphanum(i: &str) -> Result<(&str, AlphaNum), ()> {
    if let Ok((i, x)) = char::eat(i, ()) {
        if x.is_alphanumeric() {
            return Ok((i, AlphaNum(x)));
        }
    }
    Err(())
}
