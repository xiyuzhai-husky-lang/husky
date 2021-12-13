use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Type,
    TemplateType,
    TemplateFunc,
    Func,
    Pattern,
    Auto,
    Template,
}

impl ScopeKind {
    pub(crate) fn new(keyword: Keyword) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Def => todo!(),
            Keyword::Fn => todo!(),
            Keyword::Func => todo!(),
            Keyword::Pattern => todo!(),
            Keyword::Struct => todo!(),
            Keyword::Rename => todo!(),
            Keyword::Enum => todo!(),
            Keyword::Props => todo!(),
            Keyword::Tmpl => todo!(),
            Keyword::Main => todo!(),
            Keyword::Let => todo!(),
            Keyword::Var => todo!(),
            Keyword::If => todo!(),
            Keyword::Elif => todo!(),
            Keyword::Else => todo!(),
            Keyword::Switch => todo!(),
            Keyword::Case => todo!(),
            Keyword::DeFault => todo!(),
            Keyword::For => todo!(),
            Keyword::Ext => todo!(),
            Keyword::ForExt => todo!(),
            Keyword::While => todo!(),
            Keyword::Do => todo!(),
            Keyword::Break => todo!(),
            Keyword::Return => todo!(),
        }
    }
}
