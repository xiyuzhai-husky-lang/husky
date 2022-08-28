mod canonical;
mod fmt;
mod intern;
mod menu;
mod utils;
mod verifiy;

pub use canonical::*;
pub use husky_entity_kind::EntityKind;
pub use intern::{
    new_entity_route_interner, EntityRouteInterner, EntityRoutePtr, InternEntityRoute,
};
pub use menu::*;

use husky_file::FilePtr;
use husky_text::{TextRange, TextRanged};
use husky_word::{ContextualIdentifier, CustomIdentifier, Identifier, RootIdentifier};
use thin_vec::{thin_vec, ThinVec};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntityRoute {
    pub variant: EntityRouteVariant,
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
        self.root_fmt(f)
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

impl From<RootIdentifier> for EntityRouteVariant {
    fn from(ident: RootIdentifier) -> Self {
        Self::Root { ident }
    }
}

impl From<&RootIdentifier> for EntityRouteVariant {
    fn from(ident: &RootIdentifier) -> Self {
        Self::Root { ident: *ident }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityRouteVariant {
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
    TargetInputValue,
    TargetOutputType,
    Any {
        ident: CustomIdentifier,
        husky_entity_kind: EntityKind,
        // ad hoc, replace this with the type/trait it is associated to
        file: FilePtr,
        range: TextRange,
    },
    ThisType {
        // ad hoc, replace this with the type/trait it is associated to
        file: FilePtr,
        range: TextRange,
    },
}

impl EntityRoute {
    pub fn package(main: FilePtr, ident: CustomIdentifier) -> Self {
        EntityRoute {
            variant: EntityRouteVariant::Package { main, ident },
            temporal_arguments: Default::default(),
            spatial_arguments: Default::default(),
        }
    }

    pub fn ident(&self) -> Identifier {
        match self.variant {
            EntityRouteVariant::Root { ident } => ident.into(),
            EntityRouteVariant::Package { ident, .. } => ident.into(),
            EntityRouteVariant::Child { ident, .. } => ident.into(),
            EntityRouteVariant::TargetInputValue { .. } => {
                ContextualIdentifier::CrateInputValue.into()
            }
            EntityRouteVariant::TargetOutputType { .. } => {
                ContextualIdentifier::CrateOutputType.into()
            }
            EntityRouteVariant::Any { ident, .. } => ident.into(),
            EntityRouteVariant::ThisType { .. } => todo!(),
            EntityRouteVariant::TypeAsTraitMember { ident, .. } => ident.into(),
        }
    }

    pub fn subroute(
        parent: EntityRoutePtr,
        ident: CustomIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoute {
        EntityRoute {
            variant: EntityRouteVariant::Child { parent, ident },
            temporal_arguments: Default::default(),
            spatial_arguments,
        }
    }

    pub fn new_root(
        ident: RootIdentifier,
        spatial_arguments: ThinVec<SpatialArgument>,
    ) -> EntityRoute {
        EntityRoute {
            variant: EntityRouteVariant::Root { ident },
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
            variant: self.variant.clone(),
            temporal_arguments: thin_vec![],
            spatial_arguments,
        }
    }

    pub fn vec(element: EntityRoutePtr) -> Self {
        Self::new_root(RootIdentifier::Vec, thin_vec![element.into()])
    }

    pub fn array(element: EntityRoutePtr, size: usize) -> Self {
        Self::new_root(
            RootIdentifier::Array,
            thin_vec![element.into(), size.into()],
        )
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
        EntityRoute::new_root(husky_word::default_func_type(), args)
    }

    pub fn is_builtin(&self) -> bool {
        match self.variant {
            EntityRouteVariant::Root { .. } => true,
            EntityRouteVariant::Package { .. } => false,
            EntityRouteVariant::Child { parent, .. } => parent.is_builtin(),
            EntityRouteVariant::TargetInputValue { .. }
            | EntityRouteVariant::TargetOutputType { .. } => {
                todo!()
            }
            EntityRouteVariant::Any { .. } => todo!(),
            EntityRouteVariant::ThisType { .. } => todo!(),
            EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
        }
    }

    pub fn parent(&self) -> EntityRoutePtr {
        self.opt_parent().unwrap()
    }

    pub fn opt_parent(&self) -> Option<EntityRoutePtr> {
        match self.variant {
            EntityRouteVariant::Root { .. }
            | EntityRouteVariant::TargetInputValue { .. }
            | EntityRouteVariant::Package { .. }
            | EntityRouteVariant::Any { .. }
            | EntityRouteVariant::ThisType { .. } => None,
            EntityRouteVariant::TargetOutputType { .. } => todo!(),
            EntityRouteVariant::Child { parent, .. } => Some(parent),
            EntityRouteVariant::TypeAsTraitMember { ty: parent, .. } => Some(parent),
        }
    }

    pub fn entity_route_argument(&self, idx: usize) -> EntityRoutePtr {
        self.spatial_arguments[idx].take_entity_route()
    }
}

impl From<RootIdentifier> for EntityRoute {
    fn from(ident: RootIdentifier) -> Self {
        Self::new_root(ident, ThinVec::new())
    }
}
