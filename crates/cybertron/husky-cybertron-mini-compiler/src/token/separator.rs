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

impl std::fmt::Debug for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}
