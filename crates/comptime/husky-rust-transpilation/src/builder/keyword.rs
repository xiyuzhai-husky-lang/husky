use super::*;

pub enum RustKeyword {
    Fn,
    Impl,
    Pub,
    Type,
    Mod,
    Trait,
    Let,
    If,
    Else,
    While,
    Break,
    Return,
    For,
    Match,
    Struct,
    Enum,
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
            RustKeyword::If => "if",
            RustKeyword::Else => "else",
            RustKeyword::While => "while",
            RustKeyword::Break => "break",
            RustKeyword::Return => "return",
            RustKeyword::For => "for",
            RustKeyword::Match => "match",
            RustKeyword::Struct => "struct",
            RustKeyword::Enum => "enum",
        }
    }
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn keyword(&mut self, keyword: RustKeyword) {
        self.write_token_str(keyword.code());
        self.write_str("!")
    }
}
