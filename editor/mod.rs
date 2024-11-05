pub mod kid;
pub mod zipper;

pub enum Command {
    Fill(SyntaxItem),
    Forget,
    Migrate(Axis<bool>),
}

pub enum Axis<T> {
    Vertical(T),
    Horizontal(T),
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
