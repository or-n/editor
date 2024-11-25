use crate::term_from_text::name;
use eat::text::*;

#[derive(Clone)]
pub enum Command {
    Fill(SyntaxItem),
    Forget,
    Migrate(Migrate),
    Quit,
}

#[derive(Clone)]
pub enum Migrate {
    Up,
    Down(usize),
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum SyntaxItem {
    I(isize),
    Add,
    Mul,
    Pair,
    Let(String),
    If,
    IfLet,
    Parameter(String),
    Apply,
    InfixL,
    InfixR,
    Nil,
}

impl Eat<(), ()> for Command {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok(i) = "quit".drop(i) {
            return Ok((i, Command::Quit));
        }
        if let Ok((i, migrate)) = Migrate::eat(i, ()) {
            return Ok((i, Command::Migrate(migrate)));
        }
        if let Ok((i, fill)) = SyntaxItem::eat(i, ()) {
            return Ok((i, Command::Fill(fill)));
        }
        Err(())
    }
}

impl Eat<(), ()> for Migrate {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok(i) = "up".drop(i) {
            return Ok((i, Migrate::Up));
        }
        if let Ok(i) = "down".drop(i) {
            let i = ' '.drop(i)?;
            let (i, n) = u32::eat(i, ())?;
            if n < 0 {
                return Err(());
            }
            return Ok((i, Migrate::Down(n as usize)));
        }
        if let Ok(i) = "left".drop(i) {
            return Ok((i, Migrate::Left));
        }
        if let Ok(i) = "right".drop(i) {
            return Ok((i, Migrate::Right));
        }
        Err(())
    }
}

impl Eat<(), ()> for SyntaxItem {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok((i, n)) = u32::eat(i, ()) {
            return Ok((i, SyntaxItem::I(n as isize)));
        }
        if let Ok(i) = '+'.drop(i) {
            return Ok((i, SyntaxItem::Add));
        }
        if let Ok(i) = '*'.drop(i) {
            return Ok((i, SyntaxItem::Mul));
        }
        if let Ok(i) = "pair".drop(i) {
            return Ok((i, SyntaxItem::Pair));
        }
        if let Ok(i) = "let".drop(i) {
            let i = ' '.drop(i)?;
            let (i, n) = name::Term::eat(i, ()).map_err(|_| ())?;
            return Ok((i, SyntaxItem::Let(n.0)));
        }
        if let Ok(i) = "iflet".drop(i) {
            return Ok((i, SyntaxItem::IfLet));
        }
        if let Ok(i) = "if".drop(i) {
            return Ok((i, SyntaxItem::If));
        }
        if let Ok(i) = "()".drop(i) {
            return Ok((i, SyntaxItem::Nil));
        }
        if let Ok((i, n)) = name::Term::eat(i, ()) {
            return Ok((i, SyntaxItem::Parameter(n.0)));
        }
        Err(())
    }
}
