use crate::*;
use core::hash::Hash;
use husky_print_utils::{msg_once, p};
use interner::{Interned, Interner};
use paste::paste;
use std::{borrow::Borrow, ops::Deref};

pub type EntityRouteInterner = Interner<EntityRoutePtr>;

#[derive(Clone, Copy)]
pub enum EntityRoutePtr {
    Root(RootBuiltinIdentifier),
    Custom(&'static EntityRoute),
}

impl EntityRoutePtr {
    pub fn custom(&self) -> Option<&'static EntityRoute> {
        match self {
            EntityRoutePtr::Root(_) => None,
            EntityRoutePtr::Custom(scope) => Some(scope),
        }
    }

    pub fn root(self) -> RootBuiltinIdentifier {
        match self {
            EntityRoutePtr::Root(root_identifier) => root_identifier,
            _ => panic!(),
        }
    }

    pub fn is_primitive(self) -> bool {
        match self {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootBuiltinIdentifier::Void
                | RootBuiltinIdentifier::I32
                | RootBuiltinIdentifier::I64
                | RootBuiltinIdentifier::F32
                | RootBuiltinIdentifier::F64
                | RootBuiltinIdentifier::B32
                | RootBuiltinIdentifier::B64
                | RootBuiltinIdentifier::Bool => true,
                RootBuiltinIdentifier::Vec => todo!(),
                RootBuiltinIdentifier::Tuple => todo!(),
                RootBuiltinIdentifier::Debug => todo!(),
                RootBuiltinIdentifier::Mor => todo!(),
                RootBuiltinIdentifier::ThickFp => todo!(),
                RootBuiltinIdentifier::Fn => todo!(),
                RootBuiltinIdentifier::FnMut => todo!(),
                RootBuiltinIdentifier::FnOnce => todo!(),
                RootBuiltinIdentifier::Array => todo!(),
                RootBuiltinIdentifier::DatasetType => todo!(),
                RootBuiltinIdentifier::VisualType => todo!(),
                RootBuiltinIdentifier::TypeType => todo!(),
                RootBuiltinIdentifier::Trait => todo!(),
                RootBuiltinIdentifier::Module => todo!(),
                RootBuiltinIdentifier::Ref => todo!(),
                RootBuiltinIdentifier::Option => todo!(),
                _ => panic!(),
            },
            EntityRoutePtr::Custom(_) => false,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }

    pub fn intrinsic(self) -> EntityRoutePtr {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::Ref | RootBuiltinIdentifier::Option,
            } => self.entity_route_argument(0).intrinsic(),
            _ => self,
        }
    }

    pub fn is_intrinsic(self) -> bool {
        msg_once!("mutable ref");
        !self.is_option() && !self.is_eval_ref()
    }

    // todo: needs testing
    pub fn canonicalize(self) -> CanonicalTy {
        if self.is_option() {
            assert_eq!(self.spatial_arguments.len(), 1);
            self.entity_route_argument(0).canonicalize().with_option()
        } else if self.is_eval_ref() {
            assert_eq!(self.spatial_arguments.len(), 1);
            self.entity_route_argument(0).canonicalize().with_eval_ref()
        } else if self.is_temp_ref_mut() {
            // ad hoc
            assert_eq!(self.spatial_arguments.len(), 1);
            let this1 = self.entity_route_argument(0);
            assert!(this1.is_intrinsic());
            CanonicalTy::new(0, CanonicalQualifier::TempRefMut, this1)
        } else {
            CanonicalTy::new(0, CanonicalQualifier::Intrinsic, self)
        }
    }

    pub fn deref_route(self) -> EntityRoutePtr {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::Ref,
            } => self.entity_route_argument(0).deref_route(),
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::Option,
            } => todo!(),
            _ => self,
        }
    }

    pub fn is_eval_ref(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::Ref,
            } => {
                if self.temporal_arguments.len() > 0 {
                    todo!()
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    pub fn is_temp_ref_mut(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::RefMut,
            } => true,
            _ => false,
        }
    }

    pub fn is_fp(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::ThickFp,
            } => true,
            _ => false,
        }
    }

    pub fn is_option(self) -> bool {
        match self.variant {
            EntityRouteVariant::Root {
                ident: RootBuiltinIdentifier::Option,
            } => true,
            _ => false,
        }
    }

    pub fn contains_any(&self) -> bool {
        match self.variant {
            EntityRouteVariant::Child { parent, .. } => {
                if parent.contains_any() {
                    return true;
                }
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, .. } => {
                if ty.contains_any() {
                    return true;
                }
                if trai.contains_any() {
                    return true;
                }
            }
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::Any { .. } => return true,
            EntityRouteVariant::ThisType { .. } => todo!(),
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
        (**self).root_fmt(f)
    }
}

impl std::fmt::Debug for EntityRoutePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                                ident: RootBuiltinIdentifier::$reserved,
                            },
                            temporal_arguments: thin_vec![],
                            spatial_arguments: thin_vec![],
                        };
                    )*

                    match $x {
                        $(
                            RootBuiltinIdentifier::$reserved => [<$reserved:upper _ROUTE>],
                        )*
                    }
                }
            }}
        }

        match self {
            EntityRoutePtr::Root(ident) => match_root!(
                ident => Void, I32, I64, F32, F64, B32, B64, Bool, True, False, Vec, Tuple, Debug, Std, Core, Mor, ThickFp, Fn,
                FnMut, FnOnce, Array, Domains, DatasetType, VisualType, TypeType, Trait, Module,
                CloneTrait,
                CopyTrait,
                PartialEqTrait,
                EqTrait, Ref, RefMut, Option
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

impl Interned for EntityRoutePtr {
    type T = EntityRoute;

    type Owned = EntityRoute;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self {
        Self::Custom(target)
    }

    fn new_itr() -> Interner<Self> {
        new_entity_route_interner()
    }
}

impl From<RootBuiltinIdentifier> for EntityRoutePtr {
    fn from(ident: RootBuiltinIdentifier) -> Self {
        Self::Root(ident)
    }
}

impl From<&RootBuiltinIdentifier> for EntityRoutePtr {
    fn from(ident: &RootBuiltinIdentifier) -> Self {
        Self::Root(*ident)
    }
}

impl From<&EntityRoute> for EntityRoute {
    fn from(other: &EntityRoute) -> Self {
        other.clone()
    }
}

pub trait InternEntityRoute {
    fn entity_route_interner(&self) -> &EntityRouteInterner;
    fn intern_entity_route(&self, entity_route: EntityRoute) -> EntityRoutePtr {
        self.entity_route_interner().intern(entity_route)
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

    fn opt_ty(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootBuiltinIdentifier::Option.into(), thin_vec![ty.into()])
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

    fn base_route(&self, route: EntityRoutePtr) -> EntityRoutePtr {
        self.intern_entity_route(EntityRoute {
            variant: route.variant.clone(),
            temporal_arguments: Default::default(),
            spatial_arguments: Default::default(),
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
        self.route_call(RootBuiltinIdentifier::Option.into(), thin_vec![ty.into()])
    }

    fn reference(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootBuiltinIdentifier::Ref.into(), thin_vec![ty.into()])
    }

    fn vec(&self, ty: EntityRoutePtr) -> EntityRoutePtr {
        self.route_call(RootBuiltinIdentifier::Vec.into(), thin_vec![ty.into()])
    }
}

impl InternEntityRoute for EntityRouteInterner {
    fn entity_route_interner(&self) -> &EntityRouteInterner {
        self
    }
}

fn new_entity_route_interner() -> EntityRouteInterner {
    EntityRouteInterner::new_from::<RootBuiltinIdentifier>(&[
        RootBuiltinIdentifier::Void,
        RootBuiltinIdentifier::I32,
        RootBuiltinIdentifier::I64,
        RootBuiltinIdentifier::F32,
        RootBuiltinIdentifier::F64,
        RootBuiltinIdentifier::B32,
        RootBuiltinIdentifier::B64,
        RootBuiltinIdentifier::Bool,
        RootBuiltinIdentifier::True,
        RootBuiltinIdentifier::False,
        RootBuiltinIdentifier::Vec,
        RootBuiltinIdentifier::Tuple,
        RootBuiltinIdentifier::Debug,
        RootBuiltinIdentifier::Std,
        RootBuiltinIdentifier::Core,
        RootBuiltinIdentifier::ThickFp,
        RootBuiltinIdentifier::Fn,
        RootBuiltinIdentifier::FnMut,
        RootBuiltinIdentifier::FnOnce,
        RootBuiltinIdentifier::Array,
        RootBuiltinIdentifier::Domains,
        RootBuiltinIdentifier::DatasetType,
        RootBuiltinIdentifier::TypeType,
        RootBuiltinIdentifier::Module,
        RootBuiltinIdentifier::CloneTrait,
        RootBuiltinIdentifier::CopyTrait,
        RootBuiltinIdentifier::PartialEqTrait,
        RootBuiltinIdentifier::EqTrait,
    ])
}

#[test]
fn test_new_entity_route_interner() {
    let _interner = new_entity_route_interner();
}
