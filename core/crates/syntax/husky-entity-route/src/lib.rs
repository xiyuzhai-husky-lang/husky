mod cache;
mod intern;
mod menu;
mod utils;

pub use cache::{
    insert_new_ty_route, new_ty_route_cache, try_get_ty_route, ty_route_with,
    TyRouteCacheSingletonKeeper,
};
pub use entity_kind::EntityKind;
pub use intern::{
    base_route, make_route, make_subroute, make_type_as_trait_member_route,
    new_entity_route_interner, EntityRouteInterner, EntityRouteInternerSingletonKeeper,
    EntityRoutePtr, InternEntityRoute,
};
pub use menu::{entity_route_menu, new_entity_route_menu, EntityRouteMenuSingletonKeeper};

use husky_file::FilePtr;
use husky_text::{TextRange, TextRanged};
use thin_vec::{thin_vec, ThinVec};
use word::{CustomIdentifier, Identifier, RootIdentifier};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntityRoute {
    pub kind: EntityRouteKind,
    pub temporal_arguments: ThinVec<TemporalArgument>,
    pub spatial_arguments: ThinVec<SpatialArgument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RangedEntityRoute {
    pub route: EntityRoutePtr,
    pub range: TextRange,
}

impl TextRanged for RangedEntityRoute {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl std::fmt::Debug for EntityRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.kind {
            EntityRouteKind::Root { ident } => f.write_str(&ident)?,
            EntityRouteKind::Package { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::Child { parent, ident } => {
                parent.fmt(f)?;
                f.write_str("::")?;
                f.write_str(&ident)?
            }
            EntityRouteKind::Input { .. } => f.write_str("input")?,
            EntityRouteKind::Generic { ident, .. } => f.write_str(&ident)?,
            EntityRouteKind::ThisType => f.write_str("This")?,
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                f.write_str("<")?;
                ty.fmt(f)?;
                f.write_str(" as ")?;
                trai.fmt(f)?;
                f.write_str(">::")?;
                ident.fmt(f)?
            }
        };
        if self.spatial_arguments.len() > 0 {
            f.write_str("<")?;
            for (i, generic) in self.spatial_arguments.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                match generic {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(scope) => scope.fmt(f)?,
                }
            }
            f.write_str(">")?;
        }
        Ok(())
    }
}

// the actual value that is passed to the generic entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalArgument {
    Eval,
}

// the actual value that is passed to the generic entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialArgument {
    Const(usize),
    EntityRoute(EntityRoutePtr),
}

impl SpatialArgument {
    pub fn take_entity_route(&self) -> EntityRoutePtr {
        match self {
            SpatialArgument::Const(_) => panic!(),
            SpatialArgument::EntityRoute(scope) => *scope,
        }
    }
}

impl From<usize> for SpatialArgument {
    fn from(size: usize) -> Self {
        Self::Const(size)
    }
}

impl From<EntityRoutePtr> for SpatialArgument {
    fn from(entity_route: EntityRoutePtr) -> Self {
        SpatialArgument::EntityRoute(entity_route)
    }
}

impl From<RootIdentifier> for EntityRouteKind {
    fn from(ident: RootIdentifier) -> Self {
        Self::Root { ident }
    }
}

impl From<&RootIdentifier> for EntityRouteKind {
    fn from(ident: &RootIdentifier) -> Self {
        Self::Root { ident: *ident }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityRouteKind {
    Root {
        ident: RootIdentifier,
    },
    Package {
        main: FilePtr,
        ident: CustomIdentifier,
    },
    Child {
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
    },
    TypeAsTraitMember {
        ty: EntityRoutePtr,
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
    },
    Input {
        main: FilePtr,
    },
    Generic {
        ident: CustomIdentifier,
        entity_kind: EntityKind,
    },
    ThisType,
}

impl EntityRoute {
    pub fn package(main: FilePtr, ident: CustomIdentifier) -> Self {
        EntityRoute {
            kind: EntityRouteKind::Package { main, ident },
            temporal_arguments: Default::default(),
            spatial_arguments: Default::default(),
        }
    }

    pub fn ident(&self) -> Identifier {
        match self.kind {
            EntityRouteKind::Root { ident } => ident.into(),
            EntityRouteKind::Package { ident, .. } => ident.into(),
            EntityRouteKind::Child { ident, .. } => ident.into(),
            EntityRouteKind::Input { .. } => todo!(),
            EntityRouteKind::Generic { ident, .. } => ident.into(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TypeAsTraitMember { ident, .. } => ident.into(),
        }
    }

    pub fn subroute(
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::Child { parent, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        }
    }

    pub fn new_root(
        ident: RootIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoute {
        EntityRoute {
            kind: EntityRouteKind::Root { ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        }
    }

    pub fn call(
        &self,
        new_spatial_arguments: impl IntoIterator<Item = SpatialArgument>,
    ) -> EntityRoute {
        let mut spatial_arguments = self.spatial_arguments.clone();
        spatial_arguments.extend(new_spatial_arguments);
        EntityRoute {
            kind: self.kind,
            temporal_arguments: thin_vec![],
            spatial_arguments,
        }
    }

    pub fn vec(element: SpatialArgument) -> Self {
        Self::new_root(RootIdentifier::Vec, [element].into_iter().collect())
    }

    pub fn array(element: SpatialArgument, size: usize) -> Self {
        Self::new_root(RootIdentifier::Array, thin_vec![element, size.into()])
    }

    pub fn tuple_or_void(args: ThinVec<SpatialArgument>) -> Self {
        EntityRoute::new_root(
            if args.len() > 0 {
                RootIdentifier::Tuple
            } else {
                RootIdentifier::Void
            },
            args,
        )
    }

    pub fn default_func_type(args: ThinVec<SpatialArgument>) -> Self {
        EntityRoute::new_root(word::default_func_type(), args)
    }

    pub fn is_builtin(&self) -> bool {
        match self.kind {
            EntityRouteKind::Root { .. } => true,
            EntityRouteKind::Package { .. } => false,
            EntityRouteKind::Child { parent, .. } => parent.is_builtin(),
            EntityRouteKind::Input { .. } => false,
            EntityRouteKind::Generic { .. } => todo!(),
            EntityRouteKind::ThisType => todo!(),
            EntityRouteKind::TypeAsTraitMember { .. } => todo!(),
        }
    }

    pub fn parent(&self) -> EntityRoutePtr {
        self.opt_parent().unwrap()
    }

    pub fn opt_parent(&self) -> Option<EntityRoutePtr> {
        match self.kind {
            EntityRouteKind::Root { .. }
            | EntityRouteKind::Input { .. }
            | EntityRouteKind::Package { .. }
            | EntityRouteKind::Generic { .. }
            | EntityRouteKind::ThisType => None,
            EntityRouteKind::Child { parent, .. } => Some(parent),
            EntityRouteKind::TypeAsTraitMember { ty: parent, .. } => Some(parent),
        }
    }
}

impl From<RootIdentifier> for EntityRoute {
    fn from(ident: RootIdentifier) -> Self {
        Self::new_root(ident, ThinVec::new())
    }
}
