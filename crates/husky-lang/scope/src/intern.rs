use interner::Interner;

use crate::*;

use word::Reserved;

pub type ScopeInterner = Interner<Scope, ScopeId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeId {
    Builtin(BuiltinScope),
    UserDefined(u32),
}

impl From<u32> for ScopeId {
    fn from(raw: u32) -> Self {
        ScopeId::UserDefined(raw)
    }
}

impl From<BuiltinScope> for ScopeId {
    fn from(scope: BuiltinScope) -> Self {
        Self::Builtin(scope)
    }
}

pub trait InternScope {
    fn provide_scope_interner(&self) -> &ScopeInterner;
    fn scope_to_id(&self, scope: Scope) -> ScopeId {
        self.provide_scope_interner().id(scope)
    }
    fn id_to_scope(&self, id: ScopeId) -> Scope {
        self.provide_scope_interner().clone_thing(id)
    }
}

pub fn new_scope_interner() -> ScopeInterner {
    ScopeInterner::new_from(vec![
        (Reserved::I32, BuiltinScope::I32),
        (Reserved::F32, BuiltinScope::F32),
    ])
}
