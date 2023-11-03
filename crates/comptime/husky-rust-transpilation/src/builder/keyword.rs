use super::*;

pub enum RustKeyword {
    Fn,
    Impl,
    Pub,
    Type,
    Mod,
    Trait,
    Let,
}

impl RustKeyword {
    fn code(self) -> &'static str {
        match self {
            RustKeyword::Fn => "fn",
            RustKeyword::Impl => "impl",
            RustKeyword::Pub => "pub",
            RustKeyword::Type => "type",
            RustKeyword::Mod => "mod",
            RustKeyword::Trait => "trait",
            RustKeyword::Let => "let",
        }
    }
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn keyword(&mut self, keyword: RustKeyword) {
        self.write_token_str(keyword.code())
    }
}
