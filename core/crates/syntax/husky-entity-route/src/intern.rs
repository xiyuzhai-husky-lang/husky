use crate::*;
use core::hash::Hash;
use husky_text::RangedCustomIdentifier;
use interner::{Intern, Interner};
use paste::paste;
use singleton::singleton;
use std::{any::TypeId, borrow::Borrow, ops::Deref, sync::Arc};

pub type EntityRouteInterner = Interner<EntityRoute, EntityRoute, EntityRoutePtr>;

singleton! { EntityRouteInterner }

fn entity_route_interner() -> &'static EntityRouteInterner {
    __access_singleton()
}

pub fn base_route(route: EntityRoutePtr) -> EntityRoutePtr {
    entity_route_interner().intern(EntityRoute {
        kind: route.kind,
        temporal_arguments: Default::default(),
        spatial_arguments: Default::default(),
    })
}

pub fn make_route(
    route: EntityRoutePtr,
    generic_arguments: ThinVec<SpatialArgument>,
) -> EntityRoutePtr {
    let mut generics = route.spatial_arguments.clone();
    generics.extend(generic_arguments);
    entity_route_interner().intern(EntityRoute {
        kind: route.kind,
        temporal_arguments: Default::default(),
        spatial_arguments: generics,
    })
}

pub fn make_type_as_trait_member_route(
    ty: EntityRoutePtr,
    trai: EntityRoutePtr,
    ident: CustomIdentifier,
    spatial_arguments: ThinVec<SpatialArgument>,
) -> EntityRoutePtr {
    entity_route_interner().intern(EntityRoute {
        kind: EntityRouteKind::TypeAsTraitMember { ty, trai, ident },
        temporal_arguments: Default::default(),
        spatial_arguments,
    })
}

pub fn make_subroute(
    parent: EntityRoutePtr,
    ident: CustomIdentifier,
    spatial_arguments: ThinVec<SpatialArgument>,
) -> EntityRoutePtr {
    entity_route_interner().intern(EntityRoute {
        kind: EntityRouteKind::Child { parent, ident },
        temporal_arguments: Default::default(),
        spatial_arguments,
    })
}

#[derive(Clone, Copy)]
pub enum EntityRoutePtr {
    Root(RootIdentifier),
    Custom(&'static EntityRoute),
    ThisType,
}

impl EntityRoutePtr {
    pub fn custom(&self) -> Option<&'static EntityRoute> {
        match self {
            EntityRoutePtr::Root(_) => None,
            EntityRoutePtr::Custom(scope) => Some(scope),
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }

    pub fn deref_route(self) -> EntityRoutePtr {
        match self.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => self.spatial_arguments[0].take_entity_route().deref_route(),
            _ => self,
        }
    }

    pub fn is_ref(self) -> bool {
        match self.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        (**self).fmt(f)
    }
}

impl std::fmt::Debug for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
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
            EntityRoutePtr::ThisType => (),
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
                            temporal_arguments: thin_vec![],
                            spatial_arguments: thin_vec![],
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

        const THIS_TYPE_ROUTE: &EntityRoute = &EntityRoute {
            kind: EntityRouteKind::ThisType,
            temporal_arguments: thin_vec![],
            spatial_arguments: thin_vec![],
        };

        match self {
            EntityRoutePtr::Root(ident) => match_root!(
                ident => Void, I32, F32, B32, B64, Bool, True, False, Vec, Tuple, Debug, Std, Core, Mor, Fp, Fn,
                FnMut, FnOnce, Array, Domains, DatasetType, VisualType, TypeType, TraitType, ModuleType,
                CloneTrait,
                CopyTrait,
                PartialEqTrait,
                EqTrait, Ref
            ),
            EntityRoutePtr::Custom(scope) => scope,
            EntityRoutePtr::ThisType => THIS_TYPE_ROUTE,
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

impl Intern for EntityRoutePtr {
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

pub trait InternEntityRoute {
    fn scope_interner(&self) -> &EntityRouteInterner;
    fn intern_entity_route(&self, scope: EntityRoute) -> EntityRoutePtr {
        self.scope_interner().intern(scope)
    }

    fn make_route(
        &self,
        route: EntityRoutePtr,
        generic_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoutePtr {
        let mut generics = route.spatial_arguments.clone();
        generics.extend(generic_arguments);
        self.intern_entity_route(EntityRoute {
            kind: route.kind,
            temporal_arguments: Default::default(),
            spatial_arguments: generics,
        })
    }

    fn make_subroute(
        &self,
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoutePtr {
        self.intern_entity_route(EntityRoute {
            kind: EntityRouteKind::Child { parent, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        })
    }
}

pub fn new_entity_route_interner() -> Arc<EntityRouteInternerSingletonKeeper> {
    Arc::new(
        EntityRouteInterner::new_from::<RootIdentifier>(&[
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
            RootIdentifier::Array,
            RootIdentifier::Domains,
            RootIdentifier::DatasetType,
            RootIdentifier::TypeType,
            RootIdentifier::ModuleType,
            RootIdentifier::CloneTrait,
            RootIdentifier::CopyTrait,
            RootIdentifier::PartialEqTrait,
            RootIdentifier::EqTrait,
        ])
        .into(),
    )
}

#[test]
fn test_new_entity_route_interner() {
    let _interner = new_entity_route_interner();
}
