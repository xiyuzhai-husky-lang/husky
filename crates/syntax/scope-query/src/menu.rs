use crate::*;
use scope::*;
use std::sync::Arc;
use word::BuiltinIdentifier;

pub(crate) fn scope_menu(db: &dyn ScopeSalsaQueryGroup) -> Arc<ScopeMenu> {
    Arc::new(ScopeMenu {
        clone_trait: db.intern_scope(Scope {
            kind: ScopeKind::Builtin {
                ident: BuiltinIdentifier::CloneTrait,
            },
            generics: vec![],
        }),
        void_type: db.intern_scope(Scope {
            kind: ScopeKind::Builtin {
                ident: BuiltinIdentifier::Void,
            },
            generics: vec![],
        }),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeMenu {
    pub clone_trait: ScopePtr,
    pub void_type: ScopePtr,
}
