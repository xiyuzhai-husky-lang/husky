use std::ops::Deref;



#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[salsa::debug_with_db(db = TokenDataDb, jar = TokenDataJar)]
pub enum WordOpr {
    And,
    Or,
    As,
    Be,
}

impl Deref for WordOpr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl WordOpr {
    pub const fn code(&self) -> &'static str {
        match self {
            WordOpr::And => "and",
            WordOpr::Or => "or",
            WordOpr::As => "as",
            WordOpr::Be => "be",
        }
    }
}
