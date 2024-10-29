use crate::text::*;

pub struct Digit(u32);

impl Eat<(), ()> for Digit {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (text, c) = char::eat(text, ())?;
        let digit = c.to_digit(10).ok_or(())?;
        Ok((text, Digit(digit)))
    }
}

impl Eat<(), ()> for isize {
    fn eat(text: &str, _data: ()) -> Result<(&str, Self), ()> {
        let (text, digits) = Digit::eat_many(text, ());
        if digits.is_empty() {
            return Err(());
        }
        let n = digits
            .into_iter()
            .fold(0, |r, digit| r * 10 + digit.0 as isize);
        Ok((text, n))
    }
}
