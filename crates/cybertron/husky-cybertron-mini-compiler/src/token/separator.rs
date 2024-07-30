#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    Comma,
    Semicolon,
}

impl Separator {
    pub fn data(self) -> &'static str {
        match self {
            Separator::Comma => ",",
            Separator::Semicolon => ";",
        }
    }
}
