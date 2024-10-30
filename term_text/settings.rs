#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub let_: bool,
    pub apply: bool,
    pub infixl: bool,
    pub infixr: bool,
}

impl Settings {
    pub fn all(x: bool) -> Self {
        Self {
            let_: x,
            apply: x,
            infixl: x,
            infixr: x,
        }
    }
}
