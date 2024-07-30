#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LeftDelimiter(Delimiter);

impl LeftDelimiter {
    pub fn data(self) -> &'static str {
        match self.0 {}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RightDelimiter(Delimiter);

impl RightDelimiter {
    pub fn data(self) -> &'static str {
        match self.0 {}
    }
}
