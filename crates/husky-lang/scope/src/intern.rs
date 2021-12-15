use interner::Interner;

use crate::*;

pub type ScopeInterner = Interner<Scope>;

pub type ScopeId = interner::BasicId<Scope>;

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
    ScopeInterner::new(vec![])
}
