#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub apply: bool,
    pub infixl: bool,
    pub infixr: bool,
}

impl Settings {
    pub fn all(x: bool) -> Self {
        Self {
            apply: x,
            infixl: x,
            infixr: x,
        }
    }
}
