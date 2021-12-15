use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Type,
    Routine,
    TemplateType,
    TemplateRoutine,
}

impl ScopeKind {
    pub(crate) fn new(keyword: Keyword) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use => todo!(),
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Main => Some(ScopeKind::Routine),
            Keyword::Test => Some(ScopeKind::Routine),
            Keyword::Proc => Some(ScopeKind::Routine),
            Keyword::Func => Some(ScopeKind::Routine),
            Keyword::Pattern => Some(ScopeKind::Routine),
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
