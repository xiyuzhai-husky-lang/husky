use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Value,
    Type,
    Trait,
    Func,
}

impl ScopeKind {
    pub(crate) fn new(keyword: Keyword) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use => None,
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Main => Some(ScopeKind::Func),
            Keyword::Test => Some(ScopeKind::Func),
            Keyword::Proc => Some(ScopeKind::Func),
            Keyword::Func => Some(ScopeKind::Func),
            Keyword::Def => Some(ScopeKind::Func),
            Keyword::Pattern => Some(ScopeKind::Func),
            Keyword::Struct => Some(ScopeKind::Type),
            Keyword::Rename => Some(ScopeKind::Type),
            Keyword::Enum => Some(ScopeKind::Type),
            Keyword::Props => Some(ScopeKind::Type),
            Keyword::Let
            | Keyword::Var
            | Keyword::If
            | Keyword::Elif
            | Keyword::Else
            | Keyword::Switch
            | Keyword::Match
            | Keyword::Case
            | Keyword::DeFault
            | Keyword::For
            | Keyword::Ext
            | Keyword::ForExt
            | Keyword::While
            | Keyword::Do
            | Keyword::Break
            | Keyword::Return => None,
        }
    }
}
