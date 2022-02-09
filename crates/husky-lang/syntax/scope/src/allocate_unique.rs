use core::hash::Hash;
use std::{borrow::Borrow, ops::Deref};

use unique_allocator::{UniqueAllocator, UniqueAllocatorPtr};

use paste::paste;

use crate::*;

pub type UniqueScopeAllocator = UniqueAllocator<Scope, Scope, ScopePtr>;

#[derive(Clone, Copy)]
pub enum ScopePtr {
    Builtin(BuiltinIdentifier),
    Custom(&'static Scope),
}

impl ScopePtr {
    pub fn custom(&self) -> Option<&'static Scope> {
        match self {
            ScopePtr::Builtin(_) => None,
            ScopePtr::Custom(scope) => Some(scope),
        }
    }
}

impl std::fmt::Debug for ScopePtr {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl PartialEq for ScopePtr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Builtin(l), Self::Builtin(r)) => l == r,
            (Self::Custom(l), Self::Custom(r)) => (*l as *const Scope) == (*r as *const Scope),
            _ => false,
        }
    }
}

impl Eq for ScopePtr {}

impl Hash for ScopePtr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            ScopePtr::Builtin(ident) => ident.hash(state),
            ScopePtr::Custom(scope) => (*scope as *const Scope).hash(state),
        }
    }
}

impl Deref for ScopePtr {
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        macro_rules! match_builtin {
            ($x:ident => $($reserved:ident),*) => {{
                 paste! {
                    $(
                        const [<$reserved:upper _SCOPE>]: &Scope = &Scope {
                            route: ScopeRoute::Builtin {
                                ident: BuiltinIdentifier::$reserved,
                            },
                            generics: vec![],
                        };
                    )*

                    match $x {
                        $(
                            BuiltinIdentifier::$reserved => [<$reserved:upper _SCOPE>],
                        )*
                    }
                }
            }}
        }

        match self {
            ScopePtr::Builtin(ident) => match_builtin!(
                ident => Void, I32, F32, B32, B64, Bool, True, False, Vector, Tuple, Debug, Std, Core, Fp, Fn,
                FnMut, FnOnce, Array, DatasetType
            ),
            ScopePtr::Custom(scope) => scope,
        }
    }
}

impl Borrow<Scope> for ScopePtr {
    fn borrow(&self) -> &Scope {
        self.deref()
    }
}

impl From<&'static Scope> for ScopePtr {
    fn from(target: &'static Scope) -> Self {
        Self::Custom(target)
    }
}

impl UniqueAllocatorPtr for ScopePtr {
    type Thing = Scope;
}

impl From<BuiltinIdentifier> for ScopePtr {
    fn from(ident: BuiltinIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

impl From<&BuiltinIdentifier> for ScopePtr {
    fn from(ident: &BuiltinIdentifier) -> Self {
        Self::Builtin(*ident)
    }
}

impl From<&Scope> for Scope {
    fn from(other: &Scope) -> Self {
        other.clone()
    }
}

pub trait AllocateUniqueScope {
    fn scope_unique_allocator(&self) -> &UniqueScopeAllocator;
    fn intern_scope(&self, scope: Scope) -> ScopePtr {
        self.scope_unique_allocator().alloc(scope)
    }
    fn make_scope(&self, route: ScopeRoute, generics: Vec<GenericArgument>) -> ScopePtr {
        self.intern_scope(Scope { route, generics })
    }
    fn make_child_scope(
        &self,
        parent: ScopePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> ScopePtr {
        self.intern_scope(Scope {
            route: ScopeRoute::ChildScope { parent, ident },
            generics,
        })
    }
}

pub fn new_scope_unique_allocator() -> UniqueScopeAllocator {
    UniqueScopeAllocator::new_from::<BuiltinIdentifier>(&[
        BuiltinIdentifier::Void,
        BuiltinIdentifier::I32,
        BuiltinIdentifier::F32,
        BuiltinIdentifier::B32,
        BuiltinIdentifier::B64,
        BuiltinIdentifier::Bool,
        BuiltinIdentifier::True,
        BuiltinIdentifier::False,
        BuiltinIdentifier::Vector,
        BuiltinIdentifier::Tuple,
        BuiltinIdentifier::Debug,
        BuiltinIdentifier::Std,
        BuiltinIdentifier::Core,
        BuiltinIdentifier::Fp,
        BuiltinIdentifier::Fn,
        BuiltinIdentifier::FnMut,
        BuiltinIdentifier::FnOnce,
        BuiltinIdentifier::DatasetType,
    ])
}
