use std::ops::Deref;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[salsa::derive_debug_with_db]
pub enum WordOpr {
    And,
    Or,
    As,
    Of,
    Be,
}

impl Deref for WordOpr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl WordOpr {
    pub fn code(&self) -> &'static str {
        match self {
            WordOpr::And => "and",
            WordOpr::Or => "or",
            WordOpr::As => "as",
            WordOpr::Of => "of",
            WordOpr::Be => "be",
        }
    }
}
