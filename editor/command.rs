use crate::util::text::*;

pub enum Command {
    Fill(SyntaxItem),
    Forget,
    Migrate(Migrate),
    Quit,
}

pub enum Migrate {
    Up,
    Down(usize),
}

#[derive(Clone, Copy, Debug)]
pub enum SyntaxItem {
    I,
    Add,
    Mul,
    Pair,
    Let,
    If,
    IfLet,
    Parameter,
    Apply,
    InfixL,
    InfixR,
}

pub enum Error {
    Invalid,
}

impl Eat<Error, ()> for Command {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), Error> {
        if let Ok(i) = "quit".drop(i) {
            return Ok((i, Command::Quit));
        }
        if let Ok((i, migrate)) = Migrate::eat(i, ()) {
            return Ok((i, Command::Migrate(migrate)));
        }
        Err(Error::Invalid)
    }
}

impl Eat<(), ()> for Migrate {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok(i) = "up".drop(i) {
            return Ok((i, Migrate::Up));
        }
        if let Ok(i) = "down".drop(i) {
            let i = ' '.drop(i)?;
            let (i, n) = isize::eat(i, ())?;
            if n < 0 {
                return Err(());
            }
            return Ok((i, Migrate::Down(n as usize)));
        }
        Err(())
    }
}
