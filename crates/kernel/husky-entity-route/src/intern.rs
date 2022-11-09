use crate::*;
use core::hash::Hash;
use husky_print_utils::{msg_once, p};
use interner::{Internable, Interner};
use paste::paste;
use std::{borrow::Borrow, ops::Deref};

pub type EntityRouteInterner = Interner<EntityRoute>;

impl Internable for EntityRoute {
    type BorrowedRaw = *const EntityRoute;

    type Borrowed<'a> = &'a EntityRoute;

    type Interned = EntityRouteItd;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
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

    fn try_direct(&self) -> Option<Self::Interned> {
        // can be improved
        None
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static> {
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

        match itd {
            EntityRouteItd::Root(ident) => match_root!(
                ident => Void, I32, I64, F32, F64, B32, B64, Bool, True, False, Vec, Tuple, Debug, Std, Core, Mor, ThickFp, Fn,
                FnMut, FnOnce, Array, Domains, DatasetType, VisualType, TypeType, Trait, Module,
                CloneTrait,
                CopyTrait,
                PartialEqTrait,
                EqTrait, Ref, RefMut, Option
            ),
            EntityRouteItd::Custom(scope) => scope,
        }
    }

    fn to_borrowed<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itd(&'static self, id: usize) -> Self::Interned {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum EntityRouteItd {
    Root(RootBuiltinIdentifier),
    Custom(&'static EntityRoute),
}

impl EntityRouteItd {
    pub fn custom(&self) -> Option<&'static EntityRoute> {
        match self {
            EntityRouteItd::Root(_) => None,
            EntityRouteItd::Custom(scope) => Some(scope),
        }
    }

    pub fn root(self) -> RootBuiltinIdentifier {
        match self {
            EntityRouteItd::Root(root_identifier) => root_identifier,
            _ => panic!(),
        }
    }

    pub fn is_primitive(self) -> bool {
        match self {
            EntityRouteItd::Root(root_identifier) => match root_identifier {
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
            EntityRouteItd::Custom(_) => false,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?}", self)
    }

    pub fn intrinsic(self) -> EntityRouteItd {
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

    pub fn deref_route(self) -> EntityRouteItd {
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

impl std::ops::Deref for EntityRouteItd {
    type Target = EntityRoute;

    fn deref(&self) -> &Self::Target {
        EntityRoute::itd_to_borrowed(*self)
    }
}

impl std::fmt::Display for EntityRouteItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).root_fmt(f)
    }
}

impl std::fmt::Debug for EntityRouteItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).root_fmt(f)
    }
}

impl PartialEq for EntityRouteItd {
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

impl Eq for EntityRouteItd {}

impl Hash for EntityRouteItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            EntityRouteItd::Root(ident) => ident.hash(state),
            EntityRouteItd::Custom(scope) => (*scope as *const EntityRoute).hash(state),
        }
    }
}

impl From<RootBuiltinIdentifier> for EntityRouteItd {
    fn from(ident: RootBuiltinIdentifier) -> Self {
        Self::Root(ident)
    }
}

impl From<&RootBuiltinIdentifier> for EntityRouteItd {
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
    fn intern_entity_route(&self, entity_route: EntityRoute) -> EntityRouteItd {
        self.entity_route_interner().intern(entity_route)
    }

    fn route_call(
        &self,
        route: EntityRouteItd,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRouteItd {
        let mut generics = route.spatial_arguments.clone();
        generics.extend(spatial_arguments);
        self.intern_entity_route(EntityRoute {
            variant: route.variant.clone(),
            temporal_arguments: Default::default(),
            spatial_arguments: generics,
        })
    }

    fn opt_ty(&self, ty: EntityRouteItd) -> EntityRouteItd {
        self.route_call(RootBuiltinIdentifier::Option.into(), thin_vec![ty.into()])
    }

    fn subroute(
        &self,
        parent: EntityRouteItd,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRouteItd {
        self.intern_entity_route(EntityRoute {
            variant: EntityRouteVariant::Child { parent, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        })
    }

    fn base_route(&self, route: EntityRouteItd) -> EntityRouteItd {
        self.intern_entity_route(EntityRoute {
            variant: route.variant.clone(),
            temporal_arguments: Default::default(),
            spatial_arguments: Default::default(),
        })
    }

    fn ty_as_trai_subroute(
        &self,
        ty: EntityRouteItd,
        trai: EntityRouteItd,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRouteItd {
        self.intern_entity_route(EntityRoute {
            variant: EntityRouteVariant::TypeAsTraitMember { ty, trai, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        })
    }

    fn option(&self, ty: EntityRouteItd) -> EntityRouteItd {
        self.route_call(RootBuiltinIdentifier::Option.into(), thin_vec![ty.into()])
    }

    fn reference(&self, ty: EntityRouteItd) -> EntityRouteItd {
        self.route_call(RootBuiltinIdentifier::Ref.into(), thin_vec![ty.into()])
    }

    fn vec(&self, ty: EntityRouteItd) -> EntityRouteItd {
        self.route_call(RootBuiltinIdentifier::Vec.into(), thin_vec![ty.into()])
    }
}

impl InternEntityRoute for EntityRouteInterner {
    fn entity_route_interner(&self) -> &EntityRouteInterner {
        self
    }
}
