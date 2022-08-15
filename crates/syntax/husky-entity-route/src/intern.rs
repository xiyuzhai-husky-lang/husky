use crate::*;
use core::hash::Hash;
use husky_print_utils::msg_once;
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
        variant: route.variant.clone(),
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
        variant: route.variant.clone(),
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
        variant: EntityRouteVariant::TypeAsTraitMember { ty, trai, ident },
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
        variant: EntityRouteVariant::Child { parent, ident },
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

    pub fn root(self) -> RootIdentifier {
        match self {
            EntityRoutePtr::Root(root_identifier) => root_identifier,
            _ => panic!(),
        }
    }

    pub fn is_primitive(self) -> bool {
        match self {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::I64
                | RootIdentifier::F32
                | RootIdentifier::F64
                | RootIdentifier::B32
                | RootIdentifier::B64
                | RootIdentifier::Bool => true,
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::Fp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => false,
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }

    pub fn intrinsic(self) -> EntityRoutePtr {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref | RootIdentifier::Option,
            } => self.spatial_arguments[0].take_entity_route().intrinsic(),
            _ => self,
        }
    }

    pub fn is_intrinsic(self) -> bool {
        msg_once!("mutable ref");
        !self.is_option() && !self.is_ref()
    }

    // todo: needs testing
    pub fn canonicalize(self) -> CanonicalEntityRoutePtr {
        if self.is_option() {
            assert_eq!(self.spatial_arguments.len(), 1);
            let this1 = self.spatial_arguments[0].take_entity_route();
            assert!(!this1.is_option());
            if this1.is_ref() {
                assert_eq!(this1.spatial_arguments.len(), 1);
                let this2 = this1.spatial_arguments[0].take_entity_route();
                assert!(this2.is_intrinsic());
                CanonicalEntityRoutePtr {
                    intrinsic: this2,
                    is_option: true,
                    is_ref: true,
                }
            } else {
                assert!(this1.is_intrinsic());
                CanonicalEntityRoutePtr {
                    intrinsic: this1,
                    is_option: true,
                    is_ref: false,
                }
            }
        } else if self.is_ref() {
            assert_eq!(self.spatial_arguments.len(), 1);
            let this1 = self.spatial_arguments[0].take_entity_route();
            assert!(this1.is_intrinsic());
            CanonicalEntityRoutePtr {
                intrinsic: this1,
                is_option: false,
                is_ref: true,
            }
        } else {
            todo!()
        }
    }

    pub fn deref_route(self) -> EntityRoutePtr {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => self.spatial_arguments[0].take_entity_route().deref_route(),
            _ => self,
        }
    }

    pub fn is_ref(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => true,
            _ => false,
        }
    }

    pub fn is_fp(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Fp,
            } => true,
            _ => false,
        }
    }

    pub fn is_option(self) -> bool {
        match self.deref_route().variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Option,
            } => true,
            _ => false,
        }
    }

    pub fn contains_any(&self) -> bool {
        match self.variant {
            EntityRouteVariant::Child { parent, ident } => {
                if parent.contains_any() {
                    return true;
                }
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                if ty.contains_any() {
                    return true;
                }
                if trai.contains_any() {
                    return true;
                }
            }
            EntityRouteVariant::CrateInputValue => todo!(),
            EntityRouteVariant::Any { .. } => return true,
            EntityRouteVariant::ThisType => todo!(),
            _ => (),
        }
        for spatial_argument in self.spatial_arguments.iter() {
            match spatial_argument {
                SpatialArgument::Const(_) => (),
                SpatialArgument::EntityRoute(route) => {
                    if route.contains_any() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl std::fmt::Display for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        (**self).root_fmt(f)
    }
}

impl std::fmt::Debug for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        (**self).root_fmt(f)
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
                            variant: EntityRouteVariant::Root {
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
            variant: EntityRouteVariant::ThisType,
            temporal_arguments: thin_vec![],
            spatial_arguments: thin_vec![],
        };

        match self {
            EntityRoutePtr::Root(ident) => match_root!(
                ident => Void, I32, I64, F32, F64, B32, B64, Bool, True, False, Vec, Tuple, Debug, Std, Core, Mor, Fp, Fn,
                FnMut, FnOnce, Array, Domains, DatasetType, VisualType, TypeType, TraitType, ModuleType,
                CloneTrait,
                CopyTrait,
                PartialEqTrait,
                EqTrait, Ref, Option
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

    fn route_call(
        &self,
        route: EntityRoutePtr,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoutePtr {
        let mut generics = route.spatial_arguments.clone();
        generics.extend(spatial_arguments);
        self.intern_entity_route(EntityRoute {
            variant: route.variant.clone(),
            temporal_arguments: Default::default(),
            spatial_arguments: generics,
        })
    }

    fn subroute(
        &self,
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoutePtr {
        self.intern_entity_route(EntityRoute {
            variant: EntityRouteVariant::Child { parent, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        })
    }

    fn ty_as_trai_subroute(
        &self,
        ty: EntityRoutePtr,
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoutePtr {
        self.intern_entity_route(EntityRoute {
            variant: EntityRouteVariant::TypeAsTraitMember { ty, trai, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        })
    }

    fn option(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootIdentifier::Option.into(), thin_vec![ty.into()])
    }

    fn reference(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootIdentifier::Ref.into(), thin_vec![ty.into()])
    }

    fn vec(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootIdentifier::Vec.into(), thin_vec![ty.into()])
    }
}

pub fn new_entity_route_interner() -> Arc<EntityRouteInternerSingletonKeeper> {
    Arc::new(
        EntityRouteInterner::new_from::<RootIdentifier>(&[
            RootIdentifier::Void,
            RootIdentifier::I32,
            RootIdentifier::I64,
            RootIdentifier::F32,
            RootIdentifier::F64,
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
