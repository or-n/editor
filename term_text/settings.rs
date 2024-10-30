#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub apply: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { apply: true }
    }
}
