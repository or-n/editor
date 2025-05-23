use eat::*;

#[derive(Clone)]
pub enum Command {
    Migrate(Migrate),
    Mode(super::Mode),
    Quit,
}

#[derive(Clone)]
pub enum Migrate {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum SyntaxItem {
    I(isize),
    Add,
    Mul,
}

impl Eat<&str, (), ()> for Command {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok(i) = "quit".drop(i) {
            return Ok((i, Command::Quit));
        }
        if let Ok((i, migrate)) = Migrate::eat(i, ()) {
            return Ok((i, Command::Migrate(migrate)));
        }
        // if let Ok((i, fill)) = SyntaxItem::eat(i, ()) {
        //     return Ok((i, Command::Fill(fill)));
        // }
        Err(())
    }
}

impl Eat<&str, (), ()> for Migrate {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        if let Ok(i) = "up".drop(i) {
            return Ok((i, Migrate::Up));
        }
        if let Ok(i) = "down".drop(i) {
            // let i = ' '.drop(i)?;
            // let (i, n) = u32::eat(i, ())?;
            // return Ok((i, Migrate::Down(n as usize)));
            return Ok((i, Migrate::Down));
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

impl Eat<&str, (), ()> for SyntaxItem {
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
        Err(())
    }
}
