use std::borrow::Cow;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr, // ++
    Decr, // --
    Unveil,
}

impl SuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            SuffixOpr::Incr => "++".into(),
            SuffixOpr::Decr => "--".into(),
            SuffixOpr::Unveil => "?".into(),
        }
    }
}
