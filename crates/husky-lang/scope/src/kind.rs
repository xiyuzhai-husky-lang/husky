use word::Keyword;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Module,
    Type { is_generic: bool },
    Routine { is_generic: bool },
}

impl ScopeKind {
    pub(crate) fn new(keyword: Keyword, is_generic: bool) -> Option<ScopeKind> {
        match keyword {
            Keyword::Use => todo!(),
            Keyword::Mod => Some(ScopeKind::Module),
            Keyword::Main => Some(ScopeKind::Routine { is_generic }),
            Keyword::Test => Some(ScopeKind::Routine { is_generic }),
            Keyword::Proc => Some(ScopeKind::Routine { is_generic }),
            Keyword::Func => Some(ScopeKind::Routine { is_generic }),
            Keyword::Def => Some(ScopeKind::Routine { is_generic }),
            Keyword::Pattern => Some(ScopeKind::Routine { is_generic }),
            Keyword::Struct => Some(ScopeKind::Type { is_generic }),
            Keyword::Rename => Some(ScopeKind::Type { is_generic }),
            Keyword::Enum => Some(ScopeKind::Type { is_generic }),
            Keyword::Props => Some(ScopeKind::Type { is_generic }),
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

    pub fn is_generic(&self) -> bool {
        match self {
            ScopeKind::Module => false,
            ScopeKind::Type { is_generic } => *is_generic,
            ScopeKind::Routine { is_generic } => *is_generic,
        }
    }
}
