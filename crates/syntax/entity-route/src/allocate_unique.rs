use core::hash::Hash;
use std::{any::TypeId, borrow::Borrow, ops::Deref, sync::Arc};

use unique_allocator::{UniqueAllocator, UniqueAllocatorPtr};

use paste::paste;
use vm::{AnyValue, AnyValueDyn, EnumLiteralValueDyn, StaticTypeId};

use crate::*;

pub type ScopeInterner = UniqueAllocator<EntityRoute, EntityRoute, EntityRoutePtr>;

#[derive(Clone, Copy)]
pub enum EntityRoutePtr {
    Root(RootIdentifier),
    Custom(&'static EntityRoute),
}

impl<'eval> AnyValue<'eval> for EntityRoutePtr {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<EntityRoutePtr>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "ScopePtr".into()
    }

    fn snapshot(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}

impl EnumLiteralValueDyn for EntityRoutePtr {
    fn clone_as_boxed(&self) -> Box<dyn EnumLiteralValueDyn> {
        Box::new(self.clone())
    }

    fn eq_dyn(&self, other: &dyn EnumLiteralValueDyn) -> bool {
        self.eq(other.upcast_any().downcast_ref::<EntityRoutePtr>())
    }
}

impl EntityRoutePtr {
    pub fn custom(&self) -> Option<&'static EntityRoute> {
        match self {
            EntityRoutePtr::Root(_) => None,
            EntityRoutePtr::Custom(scope) => Some(scope),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl PartialEq for EntityRoutePtr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Root(l), Self::Root(r)) => l == r,
            (Self::Custom(l), Self::Custom(r)) => {
                (*l as *const EntityRoute) == (*r as *const EntityRoute)
            }
            _ => false,
        }
    }
}

impl Eq for EntityRoutePtr {}

impl Hash for EntityRoutePtr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            EntityRoutePtr::Root(ident) => ident.hash(state),
            EntityRoutePtr::Custom(scope) => (*scope as *const EntityRoute).hash(state),
        }
    }
}

impl Deref for EntityRoutePtr {
    type Target = EntityRoute;

    fn deref(&self) -> &Self::Target {
        macro_rules! match_root {
            ($x:ident => $($reserved:ident),*) => {{
                 paste! {
                    $(
                        const [<$reserved:upper _ROUTE>]: &EntityRoute = &EntityRoute {
                            kind: EntityRouteKind::Root {
                                ident: RootIdentifier::$reserved,
                            },
                            generics: vec![],
                        };
                    )*

                    match $x {
                        $(
                            RootIdentifier::$reserved => [<$reserved:upper _ROUTE>],
                        )*
                    }
                }
            }}
        }

        match self {
            EntityRoutePtr::Root(ident) => match_root!(
                ident => Void, I32, F32, B32, B64, Bool, True, False, Vec, Tuple, Debug, Std, Core, Fp, Fn,
                FnMut, FnOnce, Array, Datasets, DatasetType, Type,
                CloneTrait,
                CopyTrait,
                PartialEqTrait,
                EqTrait
            ),
            EntityRoutePtr::Custom(scope) => scope,
        }
    }
}

impl Borrow<EntityRoute> for EntityRoutePtr {
    fn borrow(&self) -> &EntityRoute {
        self.deref()
    }
}

impl From<&'static EntityRoute> for EntityRoutePtr {
    fn from(target: &'static EntityRoute) -> Self {
        Self::Custom(target)
    }
}

impl UniqueAllocatorPtr for EntityRoutePtr {
    type Thing = EntityRoute;
}

impl From<RootIdentifier> for EntityRoutePtr {
    fn from(ident: RootIdentifier) -> Self {
        Self::Root(ident)
    }
}

impl From<&RootIdentifier> for EntityRoutePtr {
    fn from(ident: &RootIdentifier) -> Self {
        Self::Root(*ident)
    }
}

impl From<&EntityRoute> for EntityRoute {
    fn from(other: &EntityRoute) -> Self {
        other.clone()
    }
}

pub trait AllocateUniqueScope {
    fn scope_unique_allocator(&self) -> &ScopeInterner;
    fn intern_scope(&self, scope: EntityRoute) -> EntityRoutePtr {
        self.scope_unique_allocator().alloc(scope)
    }
    fn make_scope(&self, route: EntityRouteKind, generics: Vec<GenericArgument>) -> EntityRoutePtr {
        self.intern_scope(EntityRoute {
            kind: route,
            generics,
        })
    }
    fn make_child_scope(
        &self,
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> EntityRoutePtr {
        self.intern_scope(EntityRoute {
            kind: EntityRouteKind::ChildScope { parent, ident },
            generics,
        })
    }
}

pub fn new_scope_unique_allocator() -> ScopeInterner {
    ScopeInterner::new_from::<RootIdentifier>(&[
        RootIdentifier::Void,
        RootIdentifier::I32,
        RootIdentifier::F32,
        RootIdentifier::B32,
        RootIdentifier::B64,
        RootIdentifier::Bool,
        RootIdentifier::True,
        RootIdentifier::False,
        RootIdentifier::Vec,
        RootIdentifier::Tuple,
        RootIdentifier::Debug,
        RootIdentifier::Std,
        RootIdentifier::Core,
        RootIdentifier::Fp,
        RootIdentifier::Fn,
        RootIdentifier::FnMut,
        RootIdentifier::FnOnce,
        RootIdentifier::DatasetType,
    ])
}
