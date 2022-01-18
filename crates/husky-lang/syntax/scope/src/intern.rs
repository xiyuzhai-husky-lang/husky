use core::hash::Hash;
use std::{borrow::Borrow, ops::Deref};

use interner::{InternId, Interner};

use paste::paste;

use crate::*;

pub type ScopeInterner = Interner<Scope, Scope, ScopeId>;

#[derive(Clone, Copy)]
pub enum ScopeId {
    Builtin(ReservedIdentifier),
    Custom(&'static Scope),
}

impl std::fmt::Debug for ScopeId {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl PartialEq for ScopeId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Builtin(l), Self::Builtin(r)) => l == r,
            (Self::Custom(l), Self::Custom(r)) => (*l as *const Scope) == (*r as *const Scope),
            _ => false,
        }
    }
}

impl Eq for ScopeId {}

impl Hash for ScopeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            ScopeId::Builtin(ident) => ident.hash(state),
            ScopeId::Custom(scope) => (*scope as *const Scope).hash(state),
        }
    }
}

impl Deref for ScopeId {
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        macro_rules! match_builtin {
            ($x:ident => $($reserved:ident),*) => {{
                 paste! {
                    $(
                        const [<$reserved:upper _SCOPE>]:&Scope = &Scope {
                            route: ScopeRoute::Reserved {
                                ident: ReservedIdentifier::$reserved,
                            },
                            generics: vec![],
                        };
                    )*

                    match $x {
                        $(
                            ReservedIdentifier::$reserved => [<$reserved:upper _SCOPE>],
                        )*
                    }
                }
            }}
        }

        match self {
            ScopeId::Builtin(ident) => match_builtin!(
                ident => Void, I32, F32, B32, B64, Bool, Vector, Tuple, Debug, Std, Core, Fp, Fn,
                FnMut, FnOnce, Array, Input, DatasetType
            ),
            ScopeId::Custom(scope) => scope,
        }
    }
}

impl Borrow<Scope> for ScopeId {
    fn borrow(&self) -> &Scope {
        self.deref()
    }
}

impl From<&'static Scope> for ScopeId {
    fn from(target: &'static Scope) -> Self {
        Self::Custom(target)
    }
}

impl InternId for ScopeId {
    type Thing = Scope;
}

impl From<ReservedIdentifier> for ScopeId {
    fn from(ident: ReservedIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

impl From<&ReservedIdentifier> for ScopeId {
    fn from(ident: &ReservedIdentifier) -> Self {
        Self::Builtin(*ident)
    }
}

impl From<&Scope> for Scope {
    fn from(other: &Scope) -> Self {
        other.clone()
    }
}

pub trait InternScope {
    fn scope_interner(&self) -> &ScopeInterner;
    fn intern_scope(&self, scope: Scope) -> ScopeId {
        self.scope_interner().intern(scope)
    }
    fn make_scope(&self, route: ScopeRoute, generics: Vec<GenericArgument>) -> ScopeId {
        self.intern_scope(Scope { route, generics })
    }
    fn make_child_scope(
        &self,
        parent: ScopeId,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> ScopeId {
        self.intern_scope(Scope {
            route: ScopeRoute::ChildScope { parent, ident },
            generics,
        })
    }
}

pub fn new_scope_interner() -> ScopeInterner {
    ScopeInterner::new_from::<ReservedIdentifier>(&[
        ReservedIdentifier::I32,
        ReservedIdentifier::F32,
        ReservedIdentifier::Vector,
        ReservedIdentifier::Tuple,
        ReservedIdentifier::Debug,
        ReservedIdentifier::Std,
        ReservedIdentifier::Core,
        ReservedIdentifier::Fp,
        ReservedIdentifier::Fn,
        ReservedIdentifier::FnMut,
        ReservedIdentifier::FnOnce,
        ReservedIdentifier::DatasetType,
    ])
}
