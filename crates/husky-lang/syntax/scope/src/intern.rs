use interner::Interner;

use crate::*;

pub type ScopeInterner = Interner<Scope, ScopeId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeId {
    Builtin(BuiltinIdentifier),
    Custom(u32),
}

impl From<u32> for ScopeId {
    fn from(raw: u32) -> Self {
        ScopeId::Custom(raw)
    }
}

impl From<BuiltinIdentifier> for ScopeId {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

pub trait InternScope {
    fn provide_scope_interner(&self) -> &ScopeInterner;
    fn intern_scope(&self, scope: Scope) -> ScopeId {
        self.provide_scope_interner().id(scope)
    }
    fn make_scope(&self, route: ScopeRoute, generics: Vec<GenericArgument>) -> ScopeId {
        self.provide_scope_interner().id(Scope { route, generics })
    }
    fn make_child_scope(
        &self,
        parent: ScopeId,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> ScopeId {
        self.provide_scope_interner().id(Scope {
            route: ScopeRoute::ChildScope(parent, ident),
            generics,
        })
    }
    fn id_to_scope(&self, id: ScopeId) -> Scope {
        self.provide_scope_interner().clone_thing(id)
    }
}

pub fn new_scope_interner() -> ScopeInterner {
    ScopeInterner::new_from(vec![
        (BuiltinIdentifier::I32, BuiltinIdentifier::I32),
        (BuiltinIdentifier::F32, BuiltinIdentifier::F32),
        (BuiltinIdentifier::Vector, BuiltinIdentifier::Vector),
        (BuiltinIdentifier::Tuple, BuiltinIdentifier::Tuple),
        (BuiltinIdentifier::Debug, BuiltinIdentifier::Debug),
        (BuiltinIdentifier::Std, BuiltinIdentifier::Std),
        (BuiltinIdentifier::Core, BuiltinIdentifier::Core),
        (BuiltinIdentifier::Fp, BuiltinIdentifier::Fp),
        (BuiltinIdentifier::Fn, BuiltinIdentifier::Fn),
        (BuiltinIdentifier::FnMut, BuiltinIdentifier::FnMut),
        (BuiltinIdentifier::FnOnce, BuiltinIdentifier::FnOnce),
    ])
}
