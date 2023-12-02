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
    StmtFor,
    ConnectionFor,
    Match,
    Struct,
    Enum,
    In,
    Loop,
    Mut,
    Ref,
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn keyword(&mut self, keyword: RustKeyword) {
        let s = match keyword {
            RustKeyword::Fn => "fn ",
            RustKeyword::Impl => "impl ",
            RustKeyword::Pub => "pub ",
            RustKeyword::Type => "type ",
            RustKeyword::Mod => "mod ",
            RustKeyword::Trait => "trait ",
            RustKeyword::Let => "let ",
            RustKeyword::If => "if ",
            RustKeyword::Else => " else ",
            RustKeyword::While => "while ",
            RustKeyword::Break => "break",
            RustKeyword::Return => "return ",
            RustKeyword::StmtFor => "for ",
            RustKeyword::ConnectionFor => " for ",
            RustKeyword::Loop => "loop ",
            RustKeyword::Match => "match ",
            RustKeyword::Struct => "struct ",
            RustKeyword::Enum => "enum ",
            RustKeyword::In => " in ",
            RustKeyword::Mut => "mut ",
            RustKeyword::Ref => "ref ",
        };
        self.write_str(s)
    }
}
